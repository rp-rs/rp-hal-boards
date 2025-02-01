//! Example of graphics on the LCD of the Waveshare RP2040-LCD-1.28
//!
//! Draws a red and green line with a blue rectangle.
//! After that it fills the screen line for line, at the end it starts over with
//! another colour, RED, GREEN and BLUE.
#![no_std]
#![no_main]

use cortex_m::delay::Delay;
use embedded_graphics::mono_font::ascii::FONT_10X20;
use embedded_graphics::mono_font::{MonoTextStyle, MonoTextStyleBuilder};
use embedded_graphics::primitives::{Arc, StrokeAlignment};
use embedded_graphics::text::{Alignment, Baseline, Text, TextStyle, TextStyleBuilder};
use embedded_hal::delay::DelayNs;
use fugit::RateExtU32;
use gc9a01a_driver::{FrameBuffer, Orientation, Region, GC9A01A};
use panic_halt as _;
use rp2040_hal::Timer;

use core::fmt::Write;
use heapless::String;

use waveshare_rp2040_touch_lcd_1_28::entry;
use waveshare_rp2040_touch_lcd_1_28::{
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
    primitives::{PrimitiveStyle, PrimitiveStyleBuilder, Rectangle},
};

const LCD_WIDTH: u32 = 240;
const LCD_HEIGHT: u32 = 240;
// Define static buffers
const BUFFER_SIZE: usize = (LCD_WIDTH * LCD_HEIGHT * 2) as usize;
// 16 FPS  Is as fast as I can update the arrow smoothly so all frames are as fast as the slowest.
const DESIRED_FRAME_DURATION_US: u32 = 1_000_000 / 16;

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

    // Initialize LCD pins
    let lcd_dc = pins.lcd_dc.into_push_pull_output();
    let lcd_cs = pins.lcd_cs.into_push_pull_output();
    let lcd_clk = pins.lcd_clk.into_function::<hal::gpio::FunctionSpi>();
    let lcd_mosi = pins.lcd_mosi.into_function::<hal::gpio::FunctionSpi>();
    let lcd_rst = pins
        .lcd_rst
        .into_push_pull_output_in_state(hal::gpio::PinState::High);
    let lcd_bl = pins
        .lcd_bl
        .into_push_pull_output_in_state(hal::gpio::PinState::Low);

    // Initialize SPI
    let spi = hal::Spi::<_, _, _, 8>::new(pac.SPI1, (lcd_mosi, lcd_clk));
    let spi = spi.init(
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
        40.MHz(),
        embedded_hal::spi::MODE_0,
    );

    // Initialize the display
    let mut display = GC9A01A::new(spi, lcd_dc, lcd_cs, lcd_rst, false, LCD_WIDTH, LCD_HEIGHT);
    //let mut delay = Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    let mut delay_wrapper = DelayWrapper::new(&mut delay);

    // Use the wrapper when initializing the display
    display.init(&mut delay_wrapper).unwrap();

    display.set_orientation(&Orientation::Landscape).unwrap();

    // Allocate the buffer in main and pass it to the FrameBuffer
    let mut background_buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    let mut background_framebuffer =
        FrameBuffer::new(&mut background_buffer, LCD_WIDTH, LCD_HEIGHT);

    let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    let mut framebuffer = FrameBuffer::new(&mut buffer, LCD_WIDTH, LCD_HEIGHT);
    background_framebuffer.clear(Rgb565::BLACK);

    // Clear the screen before turning on the backlight
    display.clear_screen(Rgb565::BLACK.into_storage()).unwrap();
    lcd_bl.into_push_pull_output_in_state(hal::gpio::PinState::High);
    delay.delay_ms(1000);

    /* Progress thingy */

    // Create styles used by the drawing operations.
    let arc_stroke = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb565::CYAN)
        .stroke_width(5)
        .stroke_alignment(StrokeAlignment::Inside)
        .build();
    let arc_position = Point::new(2, 2);
    let mut character_style = MonoTextStyle::new(&FONT_10X20, Rgb565::CYAN);
    character_style.background_color = Some(Rgb565::BLACK);
    let text_style = TextStyleBuilder::new()
        .baseline(Baseline::Middle)
        .alignment(Alignment::Center)
        .build();

    // The current progress percentage
    let mut progress = 78;

    // Initialize the timer
    let timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    'running: loop {
        //display.clear(Rgb565::BLACK).unwrap();

        let start_ticks = timer.get_counter_low();

        let sweep = progress as f32 * 360.0 / 100.0;
        let start = 90.0_f32;
        let arc_diameter = LCD_WIDTH - 4;

        let arc_bounding_region = draw_progress_arc(
            &mut framebuffer,
            arc_position,
            arc_diameter,
            start,
            sweep,
            arc_stroke,
        );
        display.store_region(arc_bounding_region).unwrap();

        let mut data = String::<32>::new(); // 32 byte string buffer

        // `write` for `heapless::String` returns an error if the buffer is full,
        // but because the buffer here is 32 bytes large, the u32 will fit with a
        // lot of space left. You can shorten the buffer if needed to save space.
        let _ = write!(data, "{progress}%").unwrap();

        // Draw centered text.
        let text_bounding_region = draw_text_with_background(
            &mut framebuffer,
            data.as_str(),
            display.bounding_box().center(),
            text_style,
            Rgb565::CYAN,
            Rgb565::BLACK,
        );

        display.store_region(text_bounding_region).unwrap();

        progress = (progress + 1) % 101;

        //Display the next set of regions.
        display.show_regions(framebuffer.get_buffer()).unwrap();
        //reset the display frame buffer from the background for the regions just displayed.
        framebuffer.copy_regions(background_framebuffer.get_buffer(), display.get_regions());
        //clear out the regions from the display so its ready to start again.
        display.clear_regions();
        // Ensure each frame takes the exact same amount of time
        let end_ticks = timer.get_counter_low();
        let frame_ticks = end_ticks - start_ticks;
        if frame_ticks < DESIRED_FRAME_DURATION_US {
            delay.delay_us(DESIRED_FRAME_DURATION_US - frame_ticks);
        }
    }
}

fn draw_text_with_background(
    framebuffer: &mut FrameBuffer,
    text: &str,
    position: Point,
    text_style: TextStyle,
    text_color: Rgb565,
    background_color: Rgb565,
) -> Region {
    let character_style = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
        .text_color(text_color)
        .background_color(background_color)
        .build();

    // Calculate the size of the text
    let text_area = Rectangle::new(
        position,
        Size::new(
            text.len() as u32 * FONT_10X20.character_size.width + 10,
            FONT_10X20.character_size.height,
        ),
    );

    // Draw the background
    Rectangle::new(position, text_area.size)
        .into_styled(PrimitiveStyle::with_fill(background_color))
        .draw(framebuffer)
        .unwrap();

    // Draw the text
    Text::with_text_style(text, position, character_style, text_style)
        .draw(framebuffer)
        .unwrap();

    // Return the bounding box
    //Added 22 width on the Region to accom0date larger numbers
    Region {
        x: text_area.top_left.x as u16,
        y: text_area.top_left.y as u16,
        width: text_area.size.width + 22,
        height: text_area.size.height,
    }
}

fn draw_progress_arc(
    framebuffer: &mut FrameBuffer,
    position: Point,
    diameter: u32,
    start: f32,
    sweep: f32,
    arc_stroke: PrimitiveStyle<Rgb565>,
) -> Region {
    // Draw an arc with a 5px wide stroke.
    Arc::new(position, diameter, start.deg(), sweep.deg())
        .into_styled(arc_stroke)
        .draw(framebuffer)
        .unwrap();

    Region {
        x: position.x as u16,
        y: position.y as u16,
        width: diameter,
        height: diameter,
    }
}
