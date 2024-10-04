//! # Pimoroni Pico LiPo 16mb (monochrome) 128x64 OLED Display with SSD1306 Driver Example
//!
//! This example assumes you got an 128x64 OLED Display with an SSD1306 driver
//! connected to your Pimoroni Pico LiPo 16mb. The +3.3V voltage source of the
//! Pimoroni Pico LiPo 16mb will be used, and the output pins 21 and 22 of the board
//! (on the lower right).
//!
//! It will demonstrate how to get an I2C device and use it with the ssd1306 crate.
//! Additionally you can also see how to format a number into a string using
//! [core::fmt].
//!
//! The following diagram will show how things should be connected.
//! These displays usually can take 3.3V up to 5V.
//!
//! ```text
//!                              VCC   SCL
//!                   /------------\    /----------\
//!                   |        GND  \  /  SDA      |
//!   _|USB|_         |    /-----\  |  |  /--------+--\
//!  |1  R 40|        |   /    __|__|__|__|___     |  |
//!  |2  P 39|        |  /    | ____________  |    |  |
//!  |3    38|- GND --+-/     | |Hello worl|  |    |  |
//!  |4  P 37|        |       | |Hello Rust|  |    |  |
//!  |5  I 36|-+3.3V -/       | |counter: 1|  |    |  |
//!  |6  C   |                | |          |  |    |  |
//!  |7  O   |                | """"""""""""  |    |  |
//!  |       |                 """""""""""""""     |  |
//!  |       |       (SSD1306 128x64 OLED Display) |  |
//!  .........                                     /  /
//!  |       |                                    /  /
//!  |     22|-GP17 I2C0 SCL---------------------/  /
//!  |20   21|-GP16 I2C0 SDA-----------------------/
//!   """""""
//! Symbols:
//!     - (+) crossing lines, not connected
//!     - (o) connected lines
//! ```
//!
//! See the `Cargo.toml` file for Copyright and license details.

#![no_std]
#![no_main]

// For string formatting.
use core::fmt::Write;

// The macro for our start-up function
use pimoroni_pico_lipo_16mb::entry;

// Time handling traits:
use fugit::RateExtU32;

// Timer for the delay on the display:
use embedded_hal::delay::DelayNs;

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use pimoroni_pico_lipo_16mb::hal::pac;

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use pimoroni_pico_lipo_16mb::hal;

// For in the graphics drawing utilities like the font
// and the drawing routines:
use embedded_graphics::{
    mono_font::{ascii::FONT_9X18_BOLD, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};

// The display driver:
use ssd1306::{prelude::*, Ssd1306};

/// Entry point to our bare-metal application.
///
/// The `#[entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables are initialised.
///
/// The function configures the RP2040 peripherals,
/// gets a handle on the I2C peripheral,
/// initializes the SSD1306 driver, initializes the text builder
/// and then draws some text on the display.
#[entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        pimoroni_pico_lipo_16mb::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins up according to their function on this particular board
    let pins = pimoroni_pico_lipo_16mb::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Configure two pins as being I²C, not GPIO
    let sda_pin: hal::gpio::Pin<_, hal::gpio::FunctionI2C, _> = pins.gpio16.reconfigure();
    let scl_pin: hal::gpio::Pin<_, hal::gpio::FunctionI2C, _> = pins.gpio17.reconfigure();

    // Create the I²C driver, using the two pre-configured pins. This will fail
    // at compile time if the pins are in the wrong mode, or if this I²C
    // peripheral isn't available on these pins!
    let i2c = hal::I2C::i2c0(
        pac.I2C0,
        sda_pin,
        scl_pin,
        400.kHz(),
        &mut pac.RESETS,
        &clocks.peripheral_clock,
    );

    // Create the I²C display interface using the standard address of 0x3C(60):
    let interface = ssd1306::I2CDisplayInterface::new(i2c);

    // You can also create the I²C display interface using the alternate address of 0x3D(61)
    //let interface = ssd1306::I2CDisplayInterface::new_alternate_address(i2c);

    // Or if your I²C display uses an custom address you can use this to set it, replace 60 with your adress it's in decimal.
    //let interface = ssd1306::I2CDisplayInterface::new_custom_address(i2c, 60);

    // Create a driver instance and initialize:
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    // Create a text style for drawing the font:
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_9X18_BOLD)
        .text_color(BinaryColor::On)
        .build();

    let mut timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    let mut count = 0;

    let mut buf = FmtBuf::new();

    loop {
        buf.reset();
        // Format some text into a static buffer:
        write!(&mut buf, "counter: {}", count).unwrap();
        count += 1;

        // Empty the display:
        display.clear();

        // Draw 3 lines of text:
        Text::with_baseline("Hello world!", Point::zero(), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();

        Text::with_baseline("Hello Rust!", Point::new(0, 16), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();

        Text::with_baseline(buf.as_str(), Point::new(0, 32), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();

        display.flush().unwrap();

        // Wait a bit:
        timer.delay_ms(500);
    }
}

/// This is a very simple buffer to pre format a short line of text
/// limited arbitrarily to 64 bytes.
struct FmtBuf {
    buf: [u8; 64],
    ptr: usize,
}

impl FmtBuf {
    fn new() -> Self {
        Self {
            buf: [0; 64],
            ptr: 0,
        }
    }

    fn reset(&mut self) {
        self.ptr = 0;
    }

    fn as_str(&self) -> &str {
        core::str::from_utf8(&self.buf[0..self.ptr]).unwrap()
    }
}

impl core::fmt::Write for FmtBuf {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let rest_len = self.buf.len() - self.ptr;
        let len = if rest_len < s.len() {
            rest_len
        } else {
            s.len()
        };
        self.buf[self.ptr..(self.ptr + len)].copy_from_slice(&s.as_bytes()[0..len]);
        self.ptr += len;
        Ok(())
    }
}

// End of file
