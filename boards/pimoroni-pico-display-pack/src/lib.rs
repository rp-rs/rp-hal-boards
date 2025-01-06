#![no_std]

pub extern crate rp2040_hal as hal;

use display_interface_spi::SPIInterface;
use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::{Rgb565, RgbColor},
};

use embedded_hal::spi::MODE_0;
use embedded_hal_0_2::{blocking::delay::DelayUs, digital::v2::OutputPin};

use fugit::RateExtU32;
pub use hal::pac;
use hal::{
    gpio::{
        bank0::{
            Gpio12, Gpio13, Gpio14, Gpio15, Gpio16, Gpio17, Gpio18, Gpio19, Gpio26, Gpio27, Gpio28,
            Gpio4, Gpio5,
        },
        FunctionI2C, FunctionSioInput, FunctionSioOutput, FunctionSpi, Pin, PinState, PullDown,
        PullNone, PullUp,
    },
    pac::{RESETS, SPI0},
    sio::SioGpioBank0,
    spi::Enabled,
    Spi,
};

#[cfg(feature = "rt")]
pub use rp2040_hal::entry;
use st7789::ST7789;

/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
#[cfg(feature = "boot2")]
#[link_section = ".boot2"]
#[no_mangle]
#[used]
pub static BOOT2_FIRMWARE: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

hal::bsp_pins! {
    Gpio4 {
        name: i2c_sda
    },
    Gpio5 {
        name: i2c_scl
    },
    Gpio12 {
        name: sw_a
    },
    Gpio13 {
        name: sw_b
    },
    Gpio14 {
        name: sw_x
    },
    Gpio15 {
        name: sw_y
    },
    Gpio16 {
        name: lcd_dc,
        aliases: { FunctionSpi, PullNone: Miso }
    },
    Gpio17 {
        name: lcd_cs,
        aliases: { FunctionSpi, PullNone: LcdCs }
    },
    Gpio18 {
        name: lcd_sclk,
        aliases: { FunctionSpi, PullNone: Sclk}
    },
    Gpio19 {
        name: lcd_mosi,
        aliases: { FunctionSpi, PullNone: Mosi }
    },
    Gpio20 {
        name: lcd_backlight
    },
    Gpio26 {
        name: led_r
    },
    Gpio27 {
        name: led_g
    },
    Gpio28 {
        name: led_b
    },
}

pub struct DummyPin;

impl OutputPin for DummyPin {
    type Error = ();

    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

pub type Screen = ST7789<
    SPIInterface<
        Spi<Enabled, SPI0, (Mosi, Sclk), 8>,
        Pin<Gpio16, FunctionSioOutput, PullNone>,
        Pin<Gpio17, FunctionSioOutput, PullNone>,
    >,
    DummyPin,
>;

pub struct Buttons {
    pub a: Pin<Gpio12, FunctionSioInput, PullUp>,
    pub b: Pin<Gpio13, FunctionSioInput, PullUp>,
    pub x: Pin<Gpio14, FunctionSioInput, PullUp>,
    pub y: Pin<Gpio15, FunctionSioInput, PullUp>,
}

pub struct RgbLed {
    pub r: Pin<Gpio26, FunctionSioOutput, PullDown>,
    pub g: Pin<Gpio27, FunctionSioOutput, PullDown>,
    pub b: Pin<Gpio28, FunctionSioOutput, PullDown>,
}

// i2c ports exposed through the qw/st connector
pub struct QwI2c {
    pub sda: Pin<Gpio4, FunctionI2C, PullUp>,
    pub scl: Pin<Gpio5, FunctionI2C, PullUp>,
}

pub struct PicoDisplayPack {
    pub buttons: Buttons,
    pub led: RgbLed,
    pub qwst: QwI2c,
    pub screen: Screen,
}

impl PicoDisplayPack {
    pub fn new(
        io: pac::IO_BANK0,
        pads: pac::PADS_BANK0,
        sio: SioGpioBank0,
        spi0: SPI0,
        resets: &mut RESETS,
        delay: &mut impl DelayUs<u32>,
    ) -> Self {
        let pins = Pins::new(io, pads, sio, resets);

        // Set up buttons
        let a = pins.sw_a.into_pull_up_input();
        let b = pins.sw_b.into_pull_up_input();
        let x = pins.sw_x.into_pull_up_input();
        let y = pins.sw_y.into_pull_up_input();

        // Set up rgb led light
        let led_r = pins.led_r.into_push_pull_output();
        let led_g = pins.led_g.into_push_pull_output();
        let led_b = pins.led_b.into_push_pull_output();

        // QW/ST port for i2c
        let i2c_sda = pins.i2c_sda.reconfigure();
        let i2c_scl = pins.i2c_scl.reconfigure();

        // Set up LCD screen through SPI interface
        let dc: Pin<Gpio16, FunctionSioOutput, PullNone> = pins.lcd_dc.reconfigure();
        let cs: Pin<Gpio17, FunctionSioOutput, PullNone> = pins.lcd_cs.reconfigure();
        let spi_sclk: Pin<Gpio18, FunctionSpi, PullNone> = pins.lcd_sclk.reconfigure();
        let spi_mosi: Pin<Gpio19, FunctionSpi, PullNone> = pins.lcd_mosi.reconfigure();

        // Set backlight on so we can actually see whats on the screen
        let _backlight = pins
            .lcd_backlight
            .into_push_pull_output_in_state(PinState::High);

        let spi: Spi<Enabled, SPI0, (Mosi, Sclk), 8> =
            Spi::new(spi0, (spi_mosi, spi_sclk)).init(resets, 125u32.MHz(), 16u32.MHz(), MODE_0);
        let spi_interface = SPIInterface::new(spi, dc, cs);
        let mut screen = ST7789::new(spi_interface, DummyPin, 320, 240);
        screen.init(delay).unwrap();
        screen
            .set_orientation(st7789::Orientation::Landscape)
            .unwrap();
        screen.clear(Rgb565::BLACK).unwrap();

        PicoDisplayPack {
            buttons: Buttons { a, b, x, y },
            led: RgbLed {
                r: led_r,
                g: led_g,
                b: led_b,
            },
            qwst: QwI2c {
                sda: i2c_sda,
                scl: i2c_scl,
            },
            screen,
        }
    }
}

pub const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;
