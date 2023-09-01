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
        aliases: { FunctionUart, PullNone: Gp0Uart0Tx }
    },
    Gpio1 {
        name: rx,
        aliases: { FunctionUart, PullNone: Gp0Uart0Rx }
    },
    Gpio16 {
        name: sda,
        aliases: { FunctionI2C, PullUp: Gp16I2C0Sda}
    },
    Gpio17 {
        name: scl,
        aliases: { FunctionI2C, PullUp: Gp17I2C0Scl }
    },

    Gpio4 {
        name: gpio4,
    },
    Gpio5 {
        name: gpio5,
    },
    Gpio6 {
        name: gpio6,
    }
    Gpio7 {
        name: gpio7,
    },

    Gpio8 {
        name: gpio8,
        aliases: { FunctionUart, PullNone: Gp8Uart0Tx }
    },
    Gpio9 {
        name: gpio9,
        aliases: { FunctionUart, PullNone: Gp9Uart0Rx}
    },

    Gpio29 {
        name: gpio29,
    },
    Gpio28 {
        name: gpio28,
    },
    Gpio27 {
        name: gpio27,
    },
    Gpio26 {
        name: gpio26,
    },

    Gpio22 {
        name: gpio22,
        aliases: { FunctionSpi, PullNone: Gp22Spi0Sck }
    },
    Gpio20 {
        name: gpio20,
        aliases: { FunctionSpi, PullNone: Gp20Spi0Rx }
    },
    Gpio23 {
        name: gpio23,
        aliases: { FunctionSpi, PullNone: Gp23Spi0Tx }
    },
    Gpio21 {
        name: gpio21,
        aliases: { FunctionSpi, PullNone: Gp21Spi0Csn }
    },

    Gpio25 {
        name: neopixel,
    },
);

pub const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;
