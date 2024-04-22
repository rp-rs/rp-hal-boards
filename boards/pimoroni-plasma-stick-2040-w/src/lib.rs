#![no_std]

pub extern crate rp2040_hal as hal;

#[cfg(feature = "rt")]
pub use rp2040_hal::entry;

/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
#[cfg(feature = "boot2")]
#[link_section = ".boot2"]
#[no_mangle]
#[used]
pub static BOOT2_FIRMWARE: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

pub use hal::pac;

hal::bsp_pins!(
    Gpio0 { name: gpio0 },
    Gpio1 { name: gpio1 },
    Gpio2 { name: gpio2 },
    Gpio3 { name: gpio3 },
    /// GPIO 2 is I2C_SDA
    Gpio4 {
        name: i2c_sda,
        aliases: { FunctionI2C: Sda }
    },
    /// GPIO 5 is I2C_SCL
    Gpio5 {
        name: ic2_scl,
        aliases: { FunctionI2C: Scl }
    },
    Gpio12 { name: gpio12 },
    Gpio13 { name: gpio13 },
    Gpio14 { name: gpio14 },
    /// GPIO 15 is connected to DAT for Apa102 and Ws2812
    Gpio15 { name: data },
    Gpio16 { name: gpio16 },
    Gpio17 { name: gpio17 },
    Gpio18 { name: gpio18 },
    Gpio19 { name: gpio19 },
    Gpio20 { name: gpio20 },
    Gpio21 { name: gpio21 },
    Gpio22 { name: gpio22 },
    Gpio26 { name: gpio26 },
    Gpio27 { name: gpio27 },
    Gpio28 { name: gpio28 },
);

pub const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;

pub const ADC_GAIN: u32 = 50;
pub const SHUNT_RESISTOR: f32 = 0.015;
