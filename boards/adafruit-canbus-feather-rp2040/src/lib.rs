#![no_std]

pub extern crate rp2040_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use hal::entry;

/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
#[cfg(feature = "boot2")]
#[link_section = ".boot2"]
#[no_mangle]
#[used]
pub static BOOT2_FIRMWARE: [u8; 256] = rp2040_boot2::BOOT_LOADER_GD25Q64CS;

pub use hal::pac;

hal::bsp_pins!(
    Gpio0 {
        name: tx,
        aliases: { FunctionUart, PullNone: UartTx }
    },
    Gpio1 {
        name: rx,
        aliases: { FunctionUart, PullNone: UartRx }
    },
    Gpio2 {
        name: sda,
        aliases: { FunctionI2C, PullUp: Sda }
    },
    Gpio3 {
        name: scl,
        aliases: { FunctionI2C, PullUp: Scl }
    },
    Gpio4 { name: d4 },
    Gpio5 { name: d5 },
    Gpio6 { name: d6 },
    Gpio7 { name: button },
    Gpio9 { name: d9 },
    Gpio10 { name: d10 },
    Gpio11 { name: d11 },
    Gpio12 { name: d12 },
    Gpio13 { name: d13 },
    Gpio20 { name: neopixel_power },
    Gpio21 { name: neopixel_data },
    Gpio14 {
        name: sclk,
        aliases: { FunctionSpi, PullNone: Sclk }
    },
    Gpio15 {
        name: mosi,
        aliases: { FunctionSpi, PullNone: Mosi }
    },
    Gpio8 {
        name: miso,
        aliases: { FunctionSpi, PullNone: Miso }
    },
    Gpio16 { name: can_standby },
    Gpio17 { name: can_tx0_rtx },
    Gpio18 { name: can_reset },
    Gpio19 { name: can_cs },
    Gpio22 { name: can_interrupt },
    Gpio23 { name: can_rx0_bf },
    Gpio24 { name: d24 },
    Gpio25 { name: d25 },
    Gpio26 { name: a0 },
    Gpio27 { name: a1 },
    Gpio28 { name: a2 },
    Gpio29 { name: a3 },
);

pub const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;
