//! # Pico USB Serial (with Interrupts) Example
//!
//! Creates a USB Serial device on a Pico board, with the USB driver running in
//! the USB interrupt.
//!
//! This will create a USB Serial device echoing anything it receives. Incoming
//! ASCII characters are converted to upercase, so you can tell it is working
//! and not just local-echo!
//!
//! See the `Cargo.toml` file for Copyright and license details.

#![no_std]
#![no_main]

// A cell to be used with `critical_section::Mutex`
use core::cell::RefCell;

// Trait required to use `writeln!(…)`
use core::fmt::Write;

// A system wide mutex synchronising IRQ & cores
use critical_section::Mutex;

// Import common embedded_hal traits
use embedded_hal::prelude::*;

// The macro for marking out exception function
use cortex_m_rt::exception;

// The macro for our start-up function
use rp_pico::entry;

// The macro for marking our interrupt functions
use rp_pico::hal::pac::interrupt;

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use rp_pico::hal::pac;

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use rp_pico::hal;

// USB Device support
use usb_device::{class_prelude::*, prelude::*};

// USB Communications Class Device support
use usbd_serial::SerialPort;

struct StaticUsb<'a> {
    usb_device: UsbDevice<'a, hal::usb::UsbBus>,
    usb_serial: SerialPort<'a, hal::usb::UsbBus>,
}

// shared with the interrupt
static STATIC_USB: Mutex<RefCell<Option<StaticUsb>>> = Mutex::new(RefCell::new(None));

/// Call the closure within a critical section passing the content of `STATIC_USB`.
///
/// This function `panic!`s if  `STATIC_USB` does not contain `Some` value.
fn do_with_usb<T>(closure: impl FnOnce(&mut StaticUsb) -> T) -> T {
    critical_section::with(|cs| {
        // Create a mutable local binding for the RefMut type wrapping our Option<StaticUsb>.
        let mut usb = STATIC_USB.borrow_ref_mut(cs);
        // Borrow the content of that binding.
        let usb = usb
            .as_mut()
            .expect("Usb must be setup before calling this function.");
        closure(usb)
    })
}

/// Entry point to our bare-metal application.
///
/// The `#[entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables are initialised.
///
/// The function
/// 1. configures the RP2040 peripherals,
/// 2. waits for USB enumeration to complete,
/// 3. runs a loop 20 times sending a message over usbd-serial before
/// 4. finally resetting to usbboot in bootrom.
#[entry]
fn main() -> ! {
    // The USB Bus Driver.
    static mut USB_BUS: Option<UsbBusAllocator<hal::usb::UsbBus>> = None;

    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let mut core = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // The delay object lets us wait for specified amounts of time (in milliseconds)
    let mut delay = hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    // Set up the USB driver
    *USB_BUS = Some(UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    )));

    // Grab a reference to the USB Bus allocator.
    let bus_ref = USB_BUS.as_mut().unwrap();

    // Set up the USB Communications Class Device driver
    let usb_serial = SerialPort::new(bus_ref);
    // Create a USB device with a fake VID and PID
    let usb_device = UsbDeviceBuilder::new(bus_ref, UsbVidPid(0x16c0, 0x27dd))
        .manufacturer("Fake company")
        .product("Serial port")
        .serial_number("TEST")
        .device_class(2) // from: https://www.usb.org/defined-class-codes
        .build();

    // Setup the shared usb items in their global storage.
    critical_section::with(|cs| {
        STATIC_USB.replace(
            cs,
            Some(StaticUsb {
                usb_device,
                usb_serial,
            }),
        );
    });

    // How many system clock ticks are there in a 100th of a second (10ms)?
    let ticks = pac::SYST::get_ticks_per_10ms();

    // Setup SysTick for a 10ms periodic interrupt.
    core.SYST.set_reload(ticks);
    core.SYST.enable_counter();
    core.SYST.enable_interrupt();

    // Enable the USB interrupt
    unsafe {
        pac::NVIC::unmask(hal::pac::Interrupt::USBCTRL_IRQ);
    };

    // wait for the usb device to be configured (aka enumerated on a host).
    while do_with_usb(|u| u.usb_device.state()) != UsbDeviceState::Configured {
        delay.delay_ms(1);
    }

    // Print a loop count every 2 seconds up to 10 times, then reset to usb boot.
    for i in 0..10 {
        delay.delay_ms(2_000);

        let mut text = heapless::String::<20>::new();
        writeln!(&mut text, "loop number: {}", i).unwrap();

        write_serial(text.as_bytes());
    }
    delay.delay_ms(2_000);
    hal::rom_data::reset_to_usb_boot(0, 0);
    unreachable!();
}

/// Writes to the serial port.
///
/// We do this with in a system wide critical-section to avoid a race hazard with the USB IRQ and
/// the other core (although this example does not use it).
fn write_serial(byte_array: &[u8]) {
    let _ = do_with_usb(|u| u.usb_serial.write(byte_array));
}

/// This function is called whenever the USB Hardware generates an Interrupt
/// Request.
///
/// We do all our USB work under interrupt, so the main thread can continue on
/// knowing nothing about USB.
#[allow(non_snake_case)]
#[interrupt]
fn USBCTRL_IRQ() {
    // Grab the global objects. This is OK as we only access them under interrupt.
    do_with_usb(|usb| {
        // Poll the USB driver with all of our supported USB Classes
        if usb.usb_device.poll(&mut [&mut usb.usb_serial]) {
            let mut buf = [0u8; 64];
            match usb.usb_serial.read(&mut buf) {
                Err(_e) => {
                    // Do nothing
                }
                Ok(0) => {
                    // Do nothing
                }
                Ok(count) => {
                    // Convert to upper case
                    buf.iter_mut().take(count).for_each(|b| {
                        b.make_ascii_uppercase();
                    });

                    // Send back to the host
                    let mut wr_ptr = &buf[..count];
                    while !wr_ptr.is_empty() {
                        let _ = usb.usb_serial.write(wr_ptr).map(|len| {
                            wr_ptr = &wr_ptr[len..];
                        });
                    }
                }
            }
        }
    });
}

#[allow(non_snake_case)]
#[exception]
fn SysTick() {
    // Keeps the usb_device updated in case there is no activity on the bus triggering a
    // USBCTRL_IRQ.
    //
    // This is required by the usb-specs and recommented by usb-device to be at least every 10ms.
    // see: https://docs.rs/usb-device/latest/usb_device/device/struct.UsbDevice.html#method.poll
    do_with_usb(|usb| usb.usb_device.poll(&mut [&mut usb.usb_serial]));
}

// End of file
