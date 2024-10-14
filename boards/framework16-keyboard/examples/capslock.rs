//! # Framework 16 Keyboard Capslock Example
//!
//! Blink the capslock LED on Framework 16 keyboards
//!
//! Note: This won't work on the numpad or macropad, as they don't have capslock
//!
//! See the `Cargo.toml` file for Copyright and license details.

#![no_std]
#![no_main]

// The macro for our start-up function
use framework16_keyboard::entry;
use framework16_keyboard::{Pins, XOSC_CRYSTAL_FREQ};

use embedded_hal::digital::{InputPin, OutputPin};

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// Pull in any important traits
use framework16_keyboard::hal::prelude::*;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use framework16_keyboard::hal::pac;

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use framework16_keyboard::hal;

/// Entry point to our bare-metal application.
///
/// The `#[entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables are initialised.
///
/// The function configures the RP2040 peripherals, then fades the LED in an
/// infinite loop.
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
        XOSC_CRYSTAL_FREQ,
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
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // The delay object lets us wait for specified amounts of time (in
    // milliseconds)
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let mut caps_led = pins.caps_led.into_push_pull_output();
    let mut sleep = pins.sleep.into_floating_input();

    loop {
        // Turn off LED in sleep
        if sleep.is_low().unwrap() {
            caps_led.set_low().unwrap();
            delay.delay_ms(100);
            continue;
        }

        // Blink LED otherwise
        caps_led.set_high().unwrap();
        delay.delay_ms(500);
        caps_led.set_low().unwrap();
        delay.delay_ms(500);
    }
}
