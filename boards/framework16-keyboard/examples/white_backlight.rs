//! # Framework 16 Keyboard/Numpad Backlight Example
//!
//! Blink the white backlight on Framework 16 keyboards or numpads
//!
//! Note this won't work on RGB keyboards or the macropad.
//!
//! See the `Cargo.toml` file for Copyright and license details.

#![no_std]
#![no_main]

// The macro for our start-up function
use framework16_keyboard::entry;
use framework16_keyboard::{Pins, XOSC_CRYSTAL_FREQ};

// GPIO traits
use embedded_hal::digital::InputPin;
use embedded_hal::pwm::SetDutyCycle;

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

// The minimum PWM value (i.e. LED brightness) we want
const LOW: u16 = 0;

// The maximum PWM value (i.e. LED brightness) we want
const HIGH: u16 = 25000;

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

    let mut sleep = pins.sleep.into_floating_input();

    // Init PWMs
    let mut pwm_slices = hal::pwm::Slices::new(pac.PWM, &mut pac.RESETS);

    // Configure PWM4
    let pwm = &mut pwm_slices.pwm4;
    pwm.set_ph_correct();
    pwm.enable();

    // Output channel B on PWM4 to the LED pin
    let channel = &mut pwm.channel_b;
    channel.output_to(pins.backlight);

    // Infinite loop, fading backlight up and down
    loop {
        // Turn off backlight in sleep
        if sleep.is_low().unwrap() {
            let _ = channel.set_duty_cycle(LOW);
            delay.delay_ms(100);
            continue;
        }

        // Ramp brightness up
        for i in (LOW..=HIGH).skip(100) {
            delay.delay_us(8);
            let _ = channel.set_duty_cycle(i);
        }

        // Ramp brightness down
        for i in (LOW..=HIGH).rev().skip(100) {
            delay.delay_us(8);
            let _ = channel.set_duty_cycle(i);
        }

        delay.delay_ms(500);
    }
}
