//! # Framework LED Matrix Module LED Test Example
//!
//! Lights up every single LED one after another.
//!
//! See the `Cargo.toml` file for Copyright and license details.

#![no_std]
#![no_main]
#![allow(clippy::needless_range_loop)]

// The macro for our start-up function
use framework_ledmatrix::entry;
use framework_ledmatrix::{Pins, XOSC_CRYSTAL_FREQ};

use embedded_hal::digital::{InputPin, OutputPin};

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// Pull in any important traits
use framework_ledmatrix::hal::prelude::*;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use framework_ledmatrix::hal::pac;

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use framework_ledmatrix::hal;

use fugit::RateExtU32;

/// Maximum brightness out of 255
///
/// 100/255 results in 250mA current draw and is plenty bright.
///  50/255 results in 160mA current draw and is plenty bright.
const MAX_BRIGHTNESS: u8 = 50;

use is31fl3741::devices::{LedMatrix, CALC_PIXEL};

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

    // Enable LED controller
    // SDB - Gpio29
    let mut led_enable = pins.sdb.into_push_pull_output();
    led_enable.set_high().unwrap();
    // INTB. Currently ignoring
    pins.intb.into_floating_input();

    let sda_pin: hal::gpio::Pin<_, hal::gpio::FunctionI2C, _> = pins.gpio26.reconfigure();
    let scl_pin: hal::gpio::Pin<_, hal::gpio::FunctionI2C, _> = pins.gpio27.reconfigure();

    let i2c = hal::I2C::i2c1(
        pac.I2C1,
        sda_pin,
        scl_pin,
        1000.kHz(),
        &mut pac.RESETS,
        &clocks.peripheral_clock,
    );

    let mut dip1 = pins.dip1.into_pull_up_input();
    let _ = dip1.is_high().unwrap();

    // Detect whether the sleep pin is connected
    // Early revisions of the hardware didn't have it wired up, if that is the
    // case we have to ignore its state.
    let mut sleep_present = false;
    let mut sleep = pins.sleep.into_pull_up_input();
    if sleep.is_low().unwrap() {
        sleep_present = true;
    }
    let mut sleep = sleep.into_pull_down_input();
    if sleep.is_high().unwrap() {
        sleep_present = true;
    }

    let mut matrix = LedMatrix::new(i2c, CALC_PIXEL);
    matrix
        .setup(&mut delay)
        .expect("failed to setup RGB controller");

    // Enable only the SW pins that we're using.
    // Otherwise driving the unused pins might result in audible noise.
    matrix
        .device
        .sw_enablement(is31fl3741::SwSetting::Sw1Sw8)
        .unwrap();

    matrix
        .set_scaling(MAX_BRIGHTNESS)
        .expect("failed to set scaling");

    loop {
        // Light up each LED, one by one
        for y in 0..matrix.device.height {
            for x in 0..matrix.device.width {
                matrix.device.pixel(x, y, 0xFF).expect("couldn't turn on");
                delay.delay_ms(100);
                matrix.device.pixel(x, y, 0).expect("couldn't turn off");

                // Reset into bootloader if system asleep
                if sleep_present && sleep.is_low().unwrap() {
                    hal::rom_data::reset_to_usb_boot(0, 0);
                }
            }
        }
    }
}
