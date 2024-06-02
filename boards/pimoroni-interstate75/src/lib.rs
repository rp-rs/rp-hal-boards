//! Board support package for the Pimoroni Interstate 75.
//!
//! Pin names follow the Interstate 75 schematic dated 08/12/2021.
//!
//! <https://shop.pimoroni.com/products/interstate-75>
//!
//! There are a few pin names that may cause confusion; for example `user_sw` is
//! connected to a button labeled "BOOT", and `ADC0..2` are connected to the expansion
//! header, so they may be used for any purpose.
//!
//! `led_r`, `led_g`, and `leg_b` pins are connected to the RGB LED on the Interstate 75
//! board; they are unrelated to the HUB75 connector so they can be used for anything.
//! Note these pins are active-low.
//!
//! Many devices number the HUB75 pins R1,G1,B1,R2,G2,B2.
//! This crate follows the labels on the Interstate 75 schematic: R0,G0,B0,R1,G1,B1.
//! If you need to connect to a device with different numbering, just treat
//! `bsp::Pins::r0` as "R1" and `bsp::Pins::r1` as "R2", etc.

#![no_std]

pub use hal::pac;
pub use rp2040_hal as hal;

#[cfg(feature = "rt")]
pub use rp2040_hal::entry;

/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
#[cfg(feature = "boot2")]
#[link_section = ".boot2"]
#[no_mangle]
#[used]
pub static BOOT2_FIRMWARE: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

hal::bsp_pins!(
    /// HUB75 interface, first red channel.
    Gpio0 { name: r0 },
    /// HUB75 interface, first green channel.
    Gpio1 { name: g0 },
    /// HUB75 interface, first blue channel.
    Gpio2 { name: b0 },
    /// HUB75 interface, second red channel.
    Gpio3 { name: r1 },
    /// HUB75 interface, second green channel.
    Gpio4 { name: g1 },
    /// HUB75 interface, second blue channel.
    Gpio5 { name: b1 },
    /// HUB75 interface, row address A.
    Gpio6 { name: row_a },
    /// HUB75 interface, row address B.
    Gpio7 { name: row_b },
    /// HUB75 interface, row address C.
    Gpio8 { name: row_c },
    /// HUB75 interface, row address D.
    Gpio9 { name: row_d },
    /// HUB75E interface, row address E.
    Gpio10 { name: row_e },
    /// HUB75 interface, clock.
    Gpio11 { name: led_clk },
    /// HUB75 interface, latch.
    Gpio12 { name: led_stb },
    /// HUB75 interface, OE#, aka LAT.
    Gpio13 { name: led_oe },

    /// Interstate 75 button A.
    Gpio14 { name: sw_a },

    //Gpio15 { name: gpio15 }, // not connected

    /// RGB LED Red on Interstate 75 PCB. Active low.
    Gpio16 { name: led_r },
    /// RGB LED Green on Interstate 75 PCB. Active low.
    Gpio17 { name: led_g },
    /// RGB LED Blue on Interstate 75 PCB. Active low.
    Gpio18 { name: led_b },

    // FIXME: add aliases for other functions for expansion header pins.

    /// I2C_INT pin on the expansion header.
    Gpio19 { name: i2c_int },
    /// I2C_SDA pin on the expansion header.
    Gpio20 { name: i2c_sda },
    /// I2C_SCL pin on the expansion header.
    Gpio21 { name: i2c_scl },

    //Gpio22 { name: gpio22 }, // not connected

    /// Interstate 75 BOOT / user switch.
    Gpio23 { name: user_sw },

    // Gpio24 { name: gpio24 }, // not connected
    // Gpio25 { name: gpio25 }, // not connected

    // FIXME: add aliases for other functions for expansion header pins.

    /// GPIO26 (ADC0 pin on the expansion header).
    Gpio26 { name: adc0 },
    /// GPIO27 (ADC1 pin on the expansion header).
    Gpio27 { name: adc1 },
    /// GPIO28 (ADC2 pin on the expansion header).
    Gpio28 { name: adc2 },

    /// GPIO29 (current sense circuit on the Interstate 75).
    Gpio29 { name: adc3 },
);

pub const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;
