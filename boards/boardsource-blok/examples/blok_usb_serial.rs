//! # USB Serial Example for the Blok
//!
//! Creates a USB Serial device on a blok board, with the USB driver running in
//! the USB interrupt and all writes being done in the main loop with the usage
//! of critical section.
//!
//! This will create a USB Serial device and then write to it.
//! A loop will write its current loop number 10 times after which
//! the Blok will reset to usb boot mode.
//! If "stop" is read by the serial device, the Blok will also
//! reset to usb boot mode.
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
use core::fmt::Write;
use heapless::String;
use panic_halt as _;
use usb_device::{
    bus::UsbBusAllocator, device::UsbDevice, device::UsbDeviceBuilder, device::UsbVidPid,
};
use usbd_serial::SerialPort;

// shared with the interrupt
static mut USB_DEVICE: Option<UsbDevice<hal::usb::UsbBus>> = None;
static mut USB_BUS: Option<UsbBusAllocator<hal::usb::UsbBus>> = None;
static mut USB_SERIAL: Option<SerialPort<hal::usb::UsbBus>> = None;

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

    let _timer = Timer::new(pac.TIMER, &mut pac.RESETS);

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

    let serial = SerialPort::new(bus_ref);
    unsafe {
        USB_SERIAL = Some(serial);
    }

    let usb_device = UsbDeviceBuilder::new(bus_ref, UsbVidPid(0x1209, 0x0001))
        .product("serial port")
        .device_class(2) // from: https://www.usb.org/defined-class-codes
        .build();
    unsafe {
        USB_DEVICE = Some(usb_device);
    }

    unsafe {
        pac::NVIC::unmask(hal::pac::Interrupt::USBCTRL_IRQ);
    }

    let core = pac::CorePeripherals::take().unwrap();
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let mut i: u8 = 0;

    loop {
        delay.delay_ms(2_000);

        let mut text: String<20> = String::new();
        writeln!(&mut text, "loop number: {}\r\n", i).unwrap();

        write_serial(text.as_bytes());

        i += 1;
        if i >= 10 {
            hal::rom_data::reset_to_usb_boot(0, 0);
        }
    }
}

/// Writes to the serial port.
///
/// We do this with interrupts disabled, to avoid a race hazard with the USB IRQ.
fn write_serial(byte_array: &[u8]) {
    let _ = critical_section::with(|_| unsafe {
        USB_SERIAL.as_mut().map(|serial| serial.write(byte_array))
    })
    .unwrap();
}

/// This function is called whenever the USB Hardware generates
/// an Interrupt Request
#[allow(non_snake_case)]
#[interrupt]
unsafe fn USBCTRL_IRQ() {
    let usb_device = USB_DEVICE.as_mut().unwrap();
    let serial = USB_SERIAL.as_mut().unwrap();

    if usb_device.poll(&mut [serial]) {
        let mut buf = [0u8; 64];

        match serial.read(&mut buf) {
            Err(_e) => {}
            Ok(_count) => {
                // gets the first 4 bytes of buf
                let mut read_text = [0u8; 4];
                read_text
                    .iter_mut()
                    .enumerate()
                    .for_each(|(i, e)| *e = buf[i]);

                if &read_text == b"stop" {
                    hal::rom_data::reset_to_usb_boot(0, 0);
                } else {
                    let _ = serial.write("write stop to reset to usb boot\r\n".as_bytes());
                }
            }
        }
    }
}
