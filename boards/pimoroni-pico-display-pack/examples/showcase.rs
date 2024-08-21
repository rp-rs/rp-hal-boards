#![no_std]
#![no_main]

use arrayvec::ArrayString;
use core::fmt::Write;
use embedded_graphics::{
    geometry::Point,
    mono_font::{iso_8859_14::FONT_8X13_BOLD, MonoTextStyle},
    pixelcolor::{Rgb565, RgbColor},
    prelude::*,
    text::Text,
};

use embedded_hal::digital::InputPin;
use embedded_hal_0_2::digital::v2::OutputPin;

use fugit::RateExtU32;
use hal::{clocks::ClockSource, pac::resets::reset, Timer, Watchdog, I2C};
use pimoroni_pico_display_pack::{entry, hal, pac, Buttons, RgbLed};

use panic_halt as _;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let cp = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    let clocks = hal::clocks::init_clocks_and_plls(
        pimoroni_pico_display_pack::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let sio = hal::Sio::new(pac.SIO);
    let sys_clock_freq = clocks.system_clock.get_freq().to_Hz();
    let mut delay = cortex_m::delay::Delay::new(cp.SYST, sys_clock_freq);
    let display_pack = pimoroni_pico_display_pack::PimoroniDisplayPack::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        pac.SPI0,
        &mut pac.RESETS,
        &mut delay,
    );
    let mut screen = display_pack.screen;

    let text_style = MonoTextStyle::new(&FONT_8X13_BOLD, Rgb565::RED);

    let Buttons {
        mut a,
        mut b,
        mut x,
        mut y,
    } = display_pack.buttons;
    let RgbLed {
        r: mut led_r,
        g: mut led_g,
        b: mut led_b,
    } = display_pack.led;
    led_r.set_high().unwrap();
    led_g.set_high().unwrap();
    led_b.set_high().unwrap();

    // i2c using qw/st port
    // sensor: https://www.adafruit.com/product/3709
    let qwst = display_pack.qwst;
    let sgp30_i2c = I2C::i2c0(
        pac.I2C0,
        qwst.sda,
        qwst.scl,
        100_000.Hz(),
        &mut pac.RESETS,
        sys_clock_freq.Hz(),
    );
    // create a separate timer to avoid delay ownership issues
    let timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);
    let mut sgp30_sensor = sgp30::Sgp30::new(sgp30_i2c, 0x58, timer);
    sgp30_sensor.init().unwrap();

    loop {
        // Clear screen and draw CO2 data
        if let Ok(meassure) = sgp30_sensor.measure() {
            screen.clear(Rgb565::BLUE).unwrap();
            let co2 = meassure.co2eq_ppm;
            let mut buffer = ArrayString::<50>::new();
            writeln!(&mut buffer, "CO2: {}", co2).unwrap();
            Text::new(&buffer, Point::new(20, 200), text_style)
                .draw(&mut screen)
                .unwrap();
        }

        // Turn on and off leds with the buttons
        if a.is_low().unwrap() {
            led_r.set_low().unwrap();
        }

        if b.is_low().unwrap() {
            led_g.set_low().unwrap();
        }

        if x.is_low().unwrap() {
            led_b.set_low().unwrap();
        }

        if y.is_low().unwrap() {
            led_r.set_high().unwrap();
            led_g.set_high().unwrap();
            led_b.set_high().unwrap();
        }

        // sleep a tiny bit
        delay.delay_ms(250);
    }
}
