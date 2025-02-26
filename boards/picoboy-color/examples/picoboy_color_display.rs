//! # Picoboy Color Display Example
//!
//! Blinks the LED on a Picoboy Color.
//!
//! This will draws a circle on the display.
//!
//! See the `Cargo.toml` file for Copyright and license details.

#![no_std]
#![no_main]

// The macro for our start-up function
use picoboy_color::entry;

// GPIO traits
use embedded_hal::digital::OutputPin;

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// Pull in any important traits
use picoboy_color::hal::prelude::*;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use picoboy_color::hal::pac;

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use picoboy_color::hal;

use display_interface_spi::SPIInterface;
use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Circle, PrimitiveStyle},
};
use fugit::RateExtU32;
use rp2040_hal::Spi;
use st7789::{Orientation, ST7789};

/// Entry point to our bare-metal application.
///
/// The `#[entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables are initialised.
///
/// The function configures the RP2040 peripherals, then draws a circle on the display.
#[entry]
fn main() -> ! {

    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        picoboy_color::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // The delay object lets us wait for specified amounts of time (in milliseconds)
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);
    let pins = picoboy_color::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Define display width and height
    const DISPLAY_WIDTH: i32 = 240;
    const DISPLAY_HEIGHT: i32 = 280;

    // Switch on backlight
    let mut backlight = pins.backlight.into_push_pull_output();
    backlight.set_high().unwrap();

    // Configure SPI pins
    let spi_sclk = pins.sck.into_function::<rp2040_hal::gpio::FunctionSpi>(); // SCK
    let spi_mosi = pins.mosi.into_function::<rp2040_hal::gpio::FunctionSpi>(); // MOSI
    let spi_miso = pins.gpio16.into_function::<rp2040_hal::gpio::FunctionSpi>(); // MISO

    // Create spi instance
    let spi = Spi::<_, _, _, 8>::new(pac.SPI0, (spi_mosi, spi_miso, spi_sclk));

    // Init spi
    let spi = spi.init(
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
        125000000u32.Hz(),
        embedded_hal::spi::MODE_3, // ST7789 requires SPI mode 3
    );

    // Configure display pins
    let dc = pins.dc.into_push_pull_output(); // Data/command pin
    let rst = pins.reset.into_push_pull_output(); // Reset pin
    let cs = pins.cs.into_push_pull_output(); // Chip select

    // Create display interface
    let di = SPIInterface::new(spi, dc, cs);

    // Create and init display
    let mut display = ST7789::new(di, rst, DISPLAY_WIDTH as u16, DISPLAY_HEIGHT as u16);

    display.init(&mut delay).unwrap();
    display
        .set_orientation(Orientation::PortraitSwapped)
        .unwrap();

    // Clear display
    display.clear(Rgb565::RED).unwrap();

    // Draw white circle
    const DIAMETER: i32 = 100;

    let circle = Circle::new(
        Point::new(
            DISPLAY_WIDTH / 2 - (DIAMETER / 2),
            DISPLAY_HEIGHT / 2 - (DIAMETER / 2),
        ),
        DIAMETER as u32,
    )
    .into_styled(PrimitiveStyle::with_stroke(Rgb565::WHITE, 5));

    circle.draw(&mut display).unwrap();

    // Red led running indicator
    let mut led_pin = pins.led_red.into_push_pull_output();

    loop {
        led_pin.set_high().unwrap();
        delay.delay_ms(500);
        led_pin.set_low().unwrap();
        delay.delay_ms(500);
    }
}

// End of file
