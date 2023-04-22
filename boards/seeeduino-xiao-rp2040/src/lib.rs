#![no_std]

pub use rp2040_hal as hal;

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
        name: rx
        aliases: { FunctionUart, PullNone: UartRx, FunctionSpi, PullNone: Csn }
    },
    Gpio2 {
        name: sck,
        aliases: { FunctionSpi, PullNone: Sck }
    },
    Gpio3 {
         name: mosi,
         aliases: { FunctionSpi, PullNone: Mosi }
    },
    Gpio4 {
         name: miso,
         aliases: { FunctionSpi, PullNone: Miso }
    },
    Gpio6 {
        name: sda,
        aliases: { FunctionI2C, PullUp: Sda }
    },
    Gpio7 {
        name: scl,
        aliases: { FunctionI2C, PullUp: Scl }
    },
    Gpio11 { name: neopixel_power },
    Gpio12 { name: neopixel_data },
    Gpio16 {
        name: led_green,
        aliases: { FunctionPwm, PullNone: LedGreenPwm }
    },
    Gpio17 {
        name: led_red,
        aliases: { FunctionPwm, PullNone: LedRedPwm }
    },
    Gpio25 {
        name: led_blue,
        aliases: { FunctionPwm, PullNone: LedBluePwm }
    },
    Gpio26 { name: a0 },
    Gpio27 { name: a1 },
    Gpio28 { name: a2 },
    Gpio29 { name: a3 },
);

pub const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;
