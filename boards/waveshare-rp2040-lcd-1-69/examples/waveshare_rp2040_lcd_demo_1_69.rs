#![no_std]
#![no_main]

use cortex_m::delay::Delay;
use embedded_hal::delay::DelayNs;

use fugit::RateExtU32;
use panic_halt as _;
use st7789v2_driver::{HORIZONTAL, ST7789V2, VERTICAL}; // for using write! macro

use waveshare_rp2040_lcd_1_69::entry;
use waveshare_rp2040_lcd_1_69::{
    hal::{
        self,
        clocks::{init_clocks_and_plls, Clock},
        pac,
        pio::PIOExt,
        watchdog::Watchdog,
        Sio,
    },
    Pins, XOSC_CRYSTAL_FREQ,
};

use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Line, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle},
};

const DEFAULT_LCD_WIDTH: u32 = 240;
const DEFAULT_LCD_HEIGHT: u32 = 280;

pub struct DelayWrapper<'a> {
    delay: &'a mut Delay,
}

impl<'a> DelayWrapper<'a> {
    pub fn new(delay: &'a mut Delay) -> Self {
        DelayWrapper { delay }
    }
}

impl<'a> DelayNs for DelayWrapper<'a> {
    fn delay_ns(&mut self, ns: u32) {
        let us = (ns + 999) / 1000; // Convert nanoseconds to microseconds
        self.delay.delay_us(us); // Use microsecond delay
    }
}

/// Main entry point for the application
#[entry]
fn main() -> ! {
    // Take ownership of peripheral instances
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Initialize watchdog
    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    // Initialize clocks and PLLs
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

    // Print the system clock frequency
    // Assuming no prescaler, timer runs at system clock frequency
    /*
    let sys_freq = clocks.system_clock.freq().to_Hz();
    let timer_freq = sys_freq;
    */

    // Initialize SIO
    let sio = Sio::new(pac.SIO);
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Set up the delay for the first core
    let sys_freq = clocks.system_clock.freq().to_Hz();
    let mut delay = Delay::new(core.SYST, sys_freq);

    let (mut _pio, _sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);

    //Initialize the Analog to Digital ADC pin for reading the resistance input.

    // Set up the ADC
    //let mut adc = Adc::new(pac.ADC, &mut pac.RESETS);
    // Configure pin 26 as an ADC pin
    //let mut adc_pin_26 = AdcPin::new(pins.gp26.into_floating_input()).unwrap();
    //let mut adc_pin = pins.gp26.into_floating_input();

    // Initialize LCD pins
    let lcd_dc = pins.gp8.into_push_pull_output();
    let lcd_cs = pins.gp9.into_push_pull_output();
    let lcd_clk = pins.gp10.into_function::<hal::gpio::FunctionSpi>();
    let lcd_mosi = pins.gp11.into_function::<hal::gpio::FunctionSpi>();
    let lcd_rst = pins
        .gp13
        .into_push_pull_output_in_state(hal::gpio::PinState::High);
    let mut _lcd_bl = pins
        .gp25
        .into_push_pull_output_in_state(hal::gpio::PinState::Low);

    // Initialize SPI
    // Initialize SPI Bus
    let spi_bus = hal::Spi::<_, _, _, 8>::new(pac.SPI1, (lcd_mosi, lcd_clk));
    let spi_bus = spi_bus.init(
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
        40.MHz(),
        embedded_hal::spi::MODE_0,
    );

    let screen_direction = VERTICAL;
    let mut lcd_width = DEFAULT_LCD_WIDTH;
    let mut lcd_height = DEFAULT_LCD_HEIGHT;
    if screen_direction == HORIZONTAL {
        lcd_width = DEFAULT_LCD_HEIGHT;
        lcd_height = DEFAULT_LCD_WIDTH;
    }
    // Initialize the display
    let mut display = ST7789V2::new(
        spi_bus,
        lcd_dc,
        lcd_cs,
        lcd_rst,
        false,
        screen_direction,
        lcd_width,
        lcd_height,
    );
    //display.init(&mut delay).unwrap();

    //let mut delay = Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    let mut delay_wrapper = DelayWrapper::new(&mut delay);

    // Use the wrapper when initializing the display
    display.init(&mut delay_wrapper).unwrap();

    // Clear the screen before turning on the backlight
    display.clear(Rgb565::BLACK).unwrap();
    _lcd_bl.into_push_pull_output_in_state(hal::gpio::PinState::High);
    delay.delay_ms(1000);

    let lcd_zero = Point::zero();
    let lcd_max_corner = Point::new((lcd_width - 1) as i32, (lcd_height - 1) as i32);

    let style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb565::BLUE)
        .build();

    Rectangle::with_corners(lcd_zero, lcd_max_corner)
        .into_styled(style)
        .draw(&mut display)
        .unwrap();
    delay.delay_ms(1000);

    let style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb565::BLACK)
        .build();

    Rectangle::with_corners(
        Point::new(1, 1),
        Point::new((lcd_width - 2) as i32, (lcd_height - 2) as i32),
    )
    .into_styled(style)
    .draw(&mut display)
    .unwrap();

    Line::new(lcd_zero, lcd_max_corner)
        .into_styled(PrimitiveStyle::with_stroke(Rgb565::RED, 1))
        .draw(&mut display)
        .unwrap();

    Line::new(
        Point::new(0, (lcd_height - 1) as i32),
        Point::new((lcd_width - 1) as i32, 0),
    )
    .into_styled(PrimitiveStyle::with_stroke(Rgb565::GREEN, 1))
    .draw(&mut display)
    .unwrap();

    // Infinite colour wheel loop
    let mut l: i32 = 0;
    let mut c = Rgb565::RED;
    loop {
        Line::new(Point::new(0, l), Point::new((lcd_width - 1) as i32, l))
            .into_styled(PrimitiveStyle::with_stroke(c, 1))
            .draw(&mut display)
            .unwrap();
        delay.delay_ms(10);
        l += 1;
        if l == lcd_height as i32 {
            l = 0;
            c = match c {
                Rgb565::RED => Rgb565::GREEN,
                Rgb565::GREEN => Rgb565::BLUE,
                _ => Rgb565::RED,
            }
        }
    }
}
