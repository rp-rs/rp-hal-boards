//! # Tufty2040 Blinky Example
//!
//! Draws a circle on the LCD screen and then blinks the user LED on the Tufty 2040.
//!
//! See the `Cargo.toml` file for Copyright and licence details.

#![no_std]
#![no_main]

use pimoroni_tufty2040 as tufty;

// The macro for our start-up function
use tufty::entry;

// GPIO traits
use embedded_hal::digital::v2::{OutputPin, PinState};

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use tufty::hal;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use hal::pac;

use hal::Timer;
use hal::Clock;
use hal::clocks::ClockSource;
use hal::gpio::{PullNone, FunctionPio0};

use tufty::DummyPin;

// A few traits required for using the CountDown timer
use embedded_hal::timer::CountDown;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::Drawable;
use embedded_graphics::pixelcolor::{Rgb565, RgbColor};
use embedded_graphics::geometry::Point;
use embedded_graphics::primitives::{Circle, Primitive, PrimitiveStyleBuilder};
use fugit::ExtU32;
use st7789::ST7789;

#[entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let cp = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        tufty::XOSC_CRYSTAL_FREQ,
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
    let pins = tufty::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Configure the timer peripheral to be a CountDown timer for our blinky delay
    let timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);
    let mut delay_timer = timer.count_down();

    let mut delay = cortex_m::delay::Delay::new(cp.SYST, clocks.system_clock.get_freq().to_Hz());

    // Set the LED to be an output
    let mut led_pin = pins.led.into_push_pull_output();

    pins.lcd_backlight.into_push_pull_output_in_state(PinState::High);
    pins.lcd_rd.into_push_pull_output_in_state(PinState::High);

    let display_data = {
        use hal::dma::DMAExt;
        use hal::pio::PIOExt;

        let dma = pac.DMA.split(&mut pac.RESETS);
        let (mut pio, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);

        let wr = pins.lcd_wr.reconfigure::<FunctionPio0, PullNone>();
        let d0 = pins.lcd_db0.reconfigure::<FunctionPio0, PullNone>();
        pins.lcd_db1.reconfigure::<FunctionPio0, PullNone>();
        pins.lcd_db2.reconfigure::<FunctionPio0, PullNone>();
        pins.lcd_db3.reconfigure::<FunctionPio0, PullNone>();
        pins.lcd_db4.reconfigure::<FunctionPio0, PullNone>();
        pins.lcd_db5.reconfigure::<FunctionPio0, PullNone>();
        pins.lcd_db6.reconfigure::<FunctionPio0, PullNone>();
        pins.lcd_db7.reconfigure::<FunctionPio0, PullNone>();

        tufty::PioDataLines::new(
            &mut pio,
            clocks.system_clock.freq(),
            wr.id(),
            d0.id(),
            sm0,
            dma.ch0,
        )
    };

    let display_interface = tufty::ParallelDisplayInterface::new(
        pins.lcd_cs.into_push_pull_output_in_state(PinState::High),
        pins.lcd_dc.into_push_pull_output_in_state(PinState::High),
        display_data,
    );

    let mut display = ST7789::new(display_interface, DummyPin, 240, 320);
    display.init(&mut delay).unwrap();
    display.clear(Rgb565::BLUE).unwrap();

    let style = PrimitiveStyleBuilder::default()
        .fill_color(Rgb565::RED)
        .build();
    Circle::new(Point::new(50, 50), 10)
        .into_styled(style)
        .draw(&mut display)
        .unwrap();

    // Blink the LED at 1 Hz
    loop {
        // LED on, and wait for 500ms
        led_pin.set_high().unwrap();
        delay_timer.start(500.millis());
        let _ = nb::block!(delay_timer.wait());

        // LED off, and wait for 500ms
        led_pin.set_low().unwrap();
        delay_timer.start(500.millis());
        let _ = nb::block!(delay_timer.wait());
    }
}
