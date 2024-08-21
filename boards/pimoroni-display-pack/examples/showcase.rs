#![no_std]
#![no_main]

use embedded_graphics::{
    geometry::Point,
    pixelcolor::{Rgb565, RgbColor},
    prelude::*,
    primitives::{Primitive, PrimitiveStyleBuilder, Triangle},
};

use embedded_hal::digital::InputPin;
use embedded_hal_0_2::digital::v2::OutputPin;

use hal::{clocks::ClockSource, Watchdog};
use pimoroni_display_pack::{entry, hal, pac, Buttons, RgbLed};

use panic_halt as _;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let cp = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    let clocks = hal::clocks::init_clocks_and_plls(
        pimoroni_display_pack::XOSC_CRYSTAL_FREQ,
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
    let display_pack = pimoroni_display_pack::PimoroniDisplayPack::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        pac.SPI0,
        &mut pac.RESETS,
        &mut delay,
    );
    let mut screen = display_pack.screen;

    let triangle = Triangle::new(Point::new(0, 0), Point::new(50, 0), Point::new(25, 50))
        .translate(Point::new(100, 100));
    let style = PrimitiveStyleBuilder::new()
        .stroke_width(1)
        .fill_color(Rgb565::RED)
        .build();

    triangle.into_styled(style).draw(&mut screen).unwrap();

    // TODO: check the schematics?
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

    // TODO: qw/st port

    loop {
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
    }
}
