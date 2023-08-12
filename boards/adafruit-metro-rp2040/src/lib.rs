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
pub static BOOT2_FIRMWARE: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

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
    Gpio2 { name: d2 },
    Gpio3 { name: d3 },
    Gpio4 { name: d4 },
    Gpio5 { name: d5 },
    Gpio6 { name: d6 },
    Gpio7 { name: d7 },
    Gpio8 { name: d8 },
    Gpio9 { name: d9 },
    Gpio10 { name: d10 },
    Gpio11 { name: d11 },
    Gpio12 { name: d12 },
    Gpio13 { name: d13 },
    Gpio14 { name: neopixel_data },
    Gpio15 { name: sd_cd, },
    Gpio16 {
        name: sda,
        aliases: { FunctionI2C, PullUp: Sda }
    },
    Gpio17 {
        name: scl,
        aliases: { FunctionI2C, PullUp: Scl }
    },
    Gpio18 {
        name: sclk,
        aliases: { FunctionSpi, PullNone: Sclk }
    },
    Gpio19 {
        name: mosi,
        aliases: { FunctionSpi, PullNone: Mosi }
    },
    Gpio20 {
        name: miso
        aliases: { FunctionSpi, PullNone: Miso }
    },
    Gpio21 { name: sdio_data1 },
    Gpio22 { name: sdio_data2 },
    Gpio23 { name: sd_cs },
    Gpio24 { name: d24 },
    Gpio25 { name: d25 },
    Gpio26 { name: a0 },
    Gpio27 { name: a1 },
    Gpio28 { name: a2 },
    Gpio29 { name: a3 },
);

pub const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;
