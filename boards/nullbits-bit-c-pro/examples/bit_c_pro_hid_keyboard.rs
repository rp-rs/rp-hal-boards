//! # Keyboard Input Example for the BIT-C PRO
//!
//! Creates a USB HID Class Keyboard device,
//! with the USB driver running in the main thread.
//!
//! When GPIO D2, D3, D4, or D5, are pulled low
//! send 'h', 'e', l', or 'o', respectively.
//!
#![no_std]
#![no_main]

use nullbits_bit_c_pro as bsp;

use bsp::{
    entry,
    hal::{
        self,
        clocks::{init_clocks_and_plls, Clock},
        pac,
        pac::interrupt,
        timer::Timer,
        watchdog::Watchdog,
        Sio,
    },
    Pins, XOSC_CRYSTAL_FREQ,
};
use embedded_hal::digital::InputPin;
use panic_halt as _;
use usb_device::{
    bus::UsbBusAllocator,
    device::{StringDescriptors, UsbDevice, UsbDeviceBuilder, UsbVidPid},
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
    let pins = Pins::new(
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
        .strings(&[StringDescriptors::default().product("keyboard input")])
        .unwrap()
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

    let mut d2 = pins.d2.into_pull_up_input();
    let mut d3 = pins.d3.into_pull_up_input();
    let mut d4 = pins.d4.into_pull_up_input();
    let mut d5 = pins.d5.into_pull_up_input();

    loop {
        let mut keycodes = [0u8; 6];

        // naively insert keycode
        if d2.is_low().unwrap_or(false) {
            keycodes[0] = 0x0b; // 'h'
        }
        if d3.is_low().unwrap_or(false) {
            keycodes[0] = 0x08 // 'e'
        }
        if d4.is_low().unwrap_or(false) {
            keycodes[0] = 0x0f // 'l'
        }
        if d5.is_low().unwrap_or(false) {
            keycodes[0] = 0x12 // 'o'
        }

        push_report(KeyboardReport {
            modifier: 0x00,
            reserved: 0x00,
            leds: 0x00,
            keycodes,
        });
        delay.delay_ms(10);
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

/// This function is called whenever the USB Hardware generates an IRQ
#[allow(non_snake_case)]
#[interrupt]
unsafe fn USBCTRL_IRQ() {
    let usb_device = USB_DEVICE.as_mut().unwrap();
    let usb_hid = USB_HID.as_mut().unwrap();
    usb_device.poll(&mut [usb_hid]);
}
