//! # Keyboard Input Example for the Blok
//!
//! Creates a USB HID Class Keyboard device on a Blok,
//! with the USB driver running in the main thread.
//!
//! It generates keyboard reports which all together
//! type the word "HELLO" on the computer.
//!
//! This behaviour will be repeated 5 times
//! after which the Blok will reset to usb boot mode.
//!
//! See the `Cargo.toml` file for Copyright and license details.

#![no_std]
#![no_main]

use boardsource_blok::{entry, hal};
use boardsource_blok::{
    hal::{
        clocks::{init_clocks_and_plls, Clock},
        pac,
        pac::interrupt,
        timer::Timer,
        watchdog::Watchdog,
        Sio,
    },
    Pins, XOSC_CRYSTAL_FREQ,
};
use panic_halt as _;
use usb_device::{
    bus::UsbBusAllocator, device::UsbDevice, device::UsbDeviceBuilder, device::UsbVidPid,
};
use usbd_hid::{descriptor::KeyboardReport, descriptor::SerializedDescriptor, hid_class::HIDClass};

// shared with the interrupt
static mut USB_BUS: Option<UsbBusAllocator<hal::usb::UsbBus>> = None;
static mut USB_HID: Option<HIDClass<hal::usb::UsbBus>> = None;
static mut USB_DEVICE: Option<UsbDevice<hal::usb::UsbBus>> = None;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();

    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    let clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let sio = Sio::new(pac.SIO);
    let _pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let _timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));
    unsafe {
        USB_BUS = Some(usb_bus);
    }

    let bus_ref = unsafe { USB_BUS.as_ref().unwrap() };

    let usb_hid = HIDClass::new(bus_ref, KeyboardReport::desc(), 10);
    unsafe {
        USB_HID = Some(usb_hid);
    }

    let usb_device = UsbDeviceBuilder::new(bus_ref, UsbVidPid(0x1209, 0x0001))
        .product("keyboard input")
        .build();
    unsafe {
        USB_DEVICE = Some(usb_device);
    }

    // enable usb interrupt
    unsafe {
        pac::NVIC::unmask(hal::pac::Interrupt::USBCTRL_IRQ);
    }

    let core = pac::CorePeripherals::take().unwrap();
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let mut i = 0;

    loop {
        // wait 5 seconds on the first loop, otherwise the first keyboard reports
        // might not be processed by the computer
        if i == 0 {
            delay.delay_ms(5_000);
        }

        delay.delay_ms(100);
        push_report(KeyboardReport {
            modifier: 0b0000_0010, // LeftShift
            reserved: 0x00,
            leds: 0x00,
            keycodes: [0x0b, 0x00, 0x00, 0x00, 0x00, 0x00], // H
        });

        delay.delay_ms(100);
        push_report(KeyboardReport {
            modifier: 0b0000_0010, // LeftShift
            reserved: 0x00,
            leds: 0x00,
            keycodes: [0x08, 0x00, 0x00, 0x00, 0x00, 0x00], // E
        });

        delay.delay_ms(100);
        push_report(KeyboardReport {
            modifier: 0b0000_0010, // LeftShift
            reserved: 0x00,
            leds: 0x00,
            keycodes: [0x0f, 0x00, 0x00, 0x00, 0x00, 0x00], // L
        });

        delay.delay_ms(100);
        push_report(KeyboardReport {
            modifier: 0x00,
            reserved: 0x00,
            leds: 0x00,
            keycodes: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // no keys pressed
        });

        delay.delay_ms(100);
        push_report(KeyboardReport {
            modifier: 0b0000_0010, // LeftShift
            reserved: 0x00,
            leds: 0x00,
            keycodes: [0x0f, 0x00, 0x00, 0x00, 0x00, 0x00], // L
        });

        delay.delay_ms(100);
        push_report(KeyboardReport {
            modifier: 0b0000_0010, // LeftShift
            reserved: 0x00,
            leds: 0x00,
            keycodes: [0x12, 0x00, 0x00, 0x00, 0x00, 0x00], // O
        });

        delay.delay_ms(100);
        push_report(KeyboardReport {
            modifier: 0x00,
            reserved: 0x00,
            leds: 0x00,
            keycodes: [0x2c, 0x00, 0x00, 0x00, 0x00, 0x00], // space
        });

        i += 1;
        if i >= 5 {
            hal::rom_data::reset_to_usb_boot(0, 0);
        }
    }
}

/// Submit a new Keyboard Report to the USB stack.
///
/// We do this with interrupts disabled, to avoid a race hazard with the USB IRQ.
fn push_report(report: KeyboardReport) {
    let _ = critical_section::with(|_| unsafe {
        // Now interrupts are disabled
        USB_HID.as_mut().map(|hid| hid.push_input(&report))
    })
    .unwrap();
}

/// This function is called whenever the USB Hardware generates
/// an Interrupt Request
#[allow(non_snake_case)]
#[interrupt]
unsafe fn USBCTRL_IRQ() {
    let usb_device = USB_DEVICE.as_mut().unwrap();
    let usb_hid = USB_HID.as_mut().unwrap();
    usb_device.poll(&mut [usb_hid]);
}
