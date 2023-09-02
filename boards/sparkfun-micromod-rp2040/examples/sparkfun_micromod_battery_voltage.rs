//! # SparkFun MicroMod Battery Voltage Example
//!
//! Continuously reads the battery voltage and prints it over defmt-rtt.
//!
//! Note that for this example to work, you need to change the runner
//! to `probe-run` (in `.cargo/config` at the root of the repository)
//! and connect to the RP2040 via SWD, preferredly via the Raspberry
//! Pi Debug Probe.
//!
//! See the `Cargo.toml` file for Copyright and license details.

#![no_std]
#![no_main]

// The macro for our start-up function
use sparkfun_micromod_rp2040 as bsp;

// Import log macros and register global logger
use defmt::*;
use defmt_rtt as _;

// Register panic handler
use panic_probe as _;
#[defmt::panic_handler]
fn panic() -> ! {
    // don't print a panic message
    // this prevents the panic message being printed *twice* when `defmt::panic` is invoked
    cortex_m::asm::udf()
}

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use bsp::hal;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use hal::pac;

// Pull in any important traits
use hal::prelude::*;

/// Entry point to our bare-metal application.
///
/// The `#[entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables are initialised.
///
/// The function configures the RP2040 peripherals, then blinks the LED in an
/// infinite loop.
#[bsp::entry]
fn main() -> ! {
    info!("Battery Voltage Example!");

    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        bsp::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // The delay object lets us wait for specified amounts of time (in
    // milliseconds)
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins up according to their function on this particular board
    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut adc = hal::Adc::new(pac.ADC, &mut pac.RESETS);
    let mut battery_voltage =
        bsp::BatteryVoltage::new(hal::adc::AdcPin::new(pins.batt_vin.into_floating_input()));

    // Print temperature once per second
    loop {
        println!("Battery: {} mV", battery_voltage.read(&mut adc));
        delay.delay_ms(1000);
    }
}

// End of file
