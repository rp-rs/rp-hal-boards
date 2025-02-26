//! # Picoboy Color Example
//!
//! Blinks the LED on a Picoboy Color.
//!
//! This will draws a controllable circle on the display.
//!
//! See the `Cargo.toml` file for Copyright and license details.

#![no_std]
#![no_main]

// The macro for our start-up function
use picoboy_color::entry;

// GPIO traits
use embedded_hal::digital::{InputPin, OutputPin};

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
    primitives::{Circle, PrimitiveStyleBuilder},
};
use fugit::RateExtU32;
use rp2040_hal::Spi;
use st7789::{Orientation, ST7789};

/// Entry point to our bare-metal application.
///
/// The `#[entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables are initialised.
///
/// The function configures the RP2040 peripherals, then draws a controllable circle.
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
    let spi_sclk = pins.sck.into_function::<rp2040_hal::gpio::FunctionSpi>();
    let spi_mosi = pins.mosi.into_function::<rp2040_hal::gpio::FunctionSpi>();
    let spi_miso = pins.gpio16.into_function::<rp2040_hal::gpio::FunctionSpi>();

    // Create spi instance
    let spi = Spi::<_, _, _, 8>::new(pac.SPI0, (spi_mosi, spi_miso, spi_sclk));

    // Init spi
    let spi = spi.init(
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
        125000000u32.Hz(),
        embedded_hal::spi::MODE_3,
    );

    // Configure display pins
    let dc = pins.dc.into_push_pull_output();
    let rst = pins.reset.into_push_pull_output();
    let cs = pins.cs.into_push_pull_output();

    // Create display interface
    let di = SPIInterface::new(spi, dc, cs);

    // Create and init display
    let mut display = ST7789::new(di, rst, DISPLAY_WIDTH as u16, DISPLAY_HEIGHT as u16);

    display.init(&mut delay).unwrap();
    display
        .set_orientation(Orientation::PortraitSwapped)
        .unwrap();

    // Configuring joystick buttons
    let mut joystick_up = pins.joystick_up.into_pull_up_input();
    let mut joystick_down = pins.joystick_down.into_pull_up_input();
    let mut joystick_left = pins.joystick_left.into_pull_up_input();
    let mut joystick_right = pins.joystick_right.into_pull_up_input();

    let mut x: i32 = DISPLAY_WIDTH / 2;
    let mut y: i32 = DISPLAY_HEIGHT / 2;

    let mut old_x = 0;
    let mut old_y = 0;

    // Clear display
    display.clear(Rgb565::BLACK).unwrap();

    loop {

        // Check entries and adjust position
        if joystick_down.is_low().unwrap() {
            y = y.saturating_add(2);
        }

        if joystick_up.is_low().unwrap() {
            y = y.saturating_sub(2);
        }

        if joystick_right.is_low().unwrap() {
            x = x.saturating_add(2);
        }

        if joystick_left.is_low().unwrap() {
            x = x.saturating_sub(2);
        }

        if x != old_x || y != old_y {

            // Only paint over the old circle with black instead of erasing the entire screen
            let old_circle = Circle::new(Point::new(old_x, old_y), 25).into_styled(
                PrimitiveStyleBuilder::new()
                    .fill_color(Rgb565::BLACK)
                    .build(),
            );
            old_circle.draw(&mut display).unwrap();

            // Draw a new circle
            let new_circle = Circle::new(Point::new(x, y), 25).into_styled(
                PrimitiveStyleBuilder::new()
                    .fill_color(Rgb565::MAGENTA)
                    .build(),
            );
            new_circle.draw(&mut display).unwrap();

            // Update positions
            old_x = x;
            old_y = y;
        }

        delay.delay_ms(50);
    }
}

// End of file
