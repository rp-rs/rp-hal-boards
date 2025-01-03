#![no_std]

//! A Hardware Abstraction Layer for the Framework LED Matrix
//!
//! This crate serves as a HAL (Hardware Abstraction Layer) for the Framework LED Matrix. Since it
//! is based on the RP2040 chip, it re-exports the [rp2040_hal] crate which contains the tooling to work with the
//! rp2040 chip.

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
    /// GPIO 0 is connected to the SLEEP# pin of the EC
    Gpio0 { name: sleep },
    /// GPIO 25 is connected to the DIP Switch #1
    Gpio25 { name: dip1 },
    /// GPIO 26 is connected to I2C SDA of the LED controller
    Gpio26 {
        name: gpio26,
        aliases: {
            /// I2C Function alias for pin [crate::Pins::gpio26].
            FunctionI2C, PullUp: Gp26I2C1Sda
        }
    },
    /// GPIO 27 is connected to I2C SCL of the LED controller
    Gpio27 {
        name: gpio27,
        aliases: {
            /// I2C Function alias for pin [crate::Pins::gpio27].
            FunctionI2C, PullUp: Gp27I2C1Scl
        }
    },
    /// GPIO 29 is connected to the INTB pin of the LED controller
    Gpio28 { name: intb },
    /// GPIO 29 is connected to the SDB pin of the LED controller
    Gpio29 { name: sdb },
);

pub const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;
