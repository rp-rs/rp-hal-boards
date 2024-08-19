//! Rainbow effect color wheel using the onboard NeoPixel on an Waveshare RP2040-Zero board
//!
//! This flows smoothly through various colours on the onboard NeoPixel.
//! Uses the `ws2812_pio` driver to control the NeoPixel, which in turns uses the
//! RP2040's PIO block.
#![no_std]
#![no_main]

use core::iter::once;
use embedded_hal::delay::DelayNs;
use palette::rgb::Rgb;
use palette::{FromColor, Hsl};
// use palette::Hsl;
use panic_halt as _;
use smart_leds::{brightness, SmartLedsWrite, RGB8};
use waveshare_rp2040_zero::entry;
use waveshare_rp2040_zero::{
    hal::{
        clocks::{init_clocks_and_plls, Clock},
        pac,
        pio::PIOExt,
        timer::Timer,
        watchdog::Watchdog,
        Sio,
    },
    Pins, XOSC_CRYSTAL_FREQ,
};
use ws2812_pio::Ws2812;

/// Entry point to our bare-metal application.
///
/// The `#[entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables are initialised.
///
/// The function configures the RP2040 peripherals, then infinitely cycles the built-in LED colour from red, to green,
/// to blue and back to red.
#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();

    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    let clocks = init_clocks_and_plls(
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

    let sio = Sio::new(pac.SIO);
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    // Configure the addressable LED
    let (mut pio, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);
    let mut ws = Ws2812::new(
        // The onboard NeoPixel is attached to GPIO pin #16 on the Waveshare RP2040-Zero.
        pins.neopixel.into_function(),
        &mut pio,
        sm0,
        clocks.peripheral_clock.freq(),
        timer.count_down(),
    );

    // Infinite colour wheel loop
    let mut hue: f64 = 0.0;
    let mut timer = timer; // rebind to force a copy of the timer
    loop {
        ws.write(brightness(
            once({
                let hsl = Hsl::<Rgb, _>::new(hue as f64, 1.0, 0.5);
                let rgb = Rgb::from_color(hsl).into_format();
                RGB8::new(rgb.red, rgb.green, rgb.blue)
            }),
            {
                // 0 is off and 1 is max brightness
                let brightness = 0.05 as f64;
                (brightness * u8::MAX as f64) as u8
            },
        ))
        .unwrap();
        hue += 360.0 / 6.0 / u8::MAX as f64;
        if hue >= 360.0 {
            hue -= 360.0;
        }
        timer.delay_ms(1)
    }
}
