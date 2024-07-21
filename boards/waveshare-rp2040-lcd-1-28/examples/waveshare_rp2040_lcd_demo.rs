//! Example of graphics on the LCD of the Waveshare RP2040-LCD-0.96
//!
//! Draws a red and green line with a blue rectangle.
//! After that it fills the screen line for line, at the end it starts over with
//! another colour, RED, GREEN and BLUE.
#![no_std]
#![no_main]

mod gc9a01a_driver;
mod frame_buffer;

use cortex_m::delay::Delay;
use embedded_graphics::primitives::Line;
use fugit::RateExtU32;
use frame_buffer::FrameBuffer;
use panic_halt as _;

use waveshare_rp2040_lcd_1_28::entry;
use waveshare_rp2040_lcd_1_28::{
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

use embedded_hal::PwmPin;
use embedded_hal::digital::v2::OutputPin;

use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, Triangle},
    image::{ImageRaw, Image},
};
use libm::{cos,sin};
use gc9a01a_driver::{Orientation, GC9A01A};

const LCD_WIDTH: u32 = 240;
const LCD_HEIGHT: u32 = 240;

// Define static buffers
static mut FRAME_BUFFER_1: [u8; (LCD_WIDTH * LCD_HEIGHT * 2) as usize] = [0; (LCD_WIDTH * LCD_HEIGHT * 2) as usize];
static mut FRAME_BUFFER_2: [u8; (LCD_WIDTH * LCD_HEIGHT * 2) as usize] = [0; (LCD_WIDTH * LCD_HEIGHT * 2) as usize];


#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

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

    // Set up the delay for the first core.
    let sys_freq = clocks.system_clock.freq().to_Hz();
    let mut delay = Delay::new(core.SYST, sys_freq);

    let (mut _pio, _sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);

    // https://www.waveshare.com/wiki/RP2040-LCD-0.96
    // ST7735S LCD
    let lcd_dc = pins.gp8.into_push_pull_output();
    let lcd_cs = pins.gp9.into_push_pull_output();
    let lcd_clk = pins.gp10.into_function::<hal::gpio::FunctionSpi>();
    let lcd_mosi = pins.gp11.into_function::<hal::gpio::FunctionSpi>();
    let lcd_rst = pins
        .gp12
        .into_push_pull_output_in_state(hal::gpio::PinState::High);

        let mut _lcd_bl = pins
        .gp25
        .into_push_pull_output_in_state(hal::gpio::PinState::Low);
    
    let spi = hal::Spi::<_, _, _, 8>::new(pac.SPI1, (lcd_mosi, lcd_clk));

    // Exchange the uninitialised SPI driver for an initialised one
    let spi = spi.init(
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
        40.MHz(),
        embedded_hal::spi::MODE_0,
    );

    // LCD is a 65K IPS LCD 160x80, color order is BGR and a offset 1,26 pixel.
    // LCD controller can correct this by settings the order bit (bit 3) in MADCTL register.
    // Also the colours are inverted, LCD controller can also correct this by writing to INVON register with no paramters.
    // All this is handled by the ST7735 crate.
    let mut display = GC9A01A::new(spi, lcd_dc, lcd_cs, lcd_rst, false, true, LCD_WIDTH, LCD_HEIGHT);

    display.init(&mut delay).unwrap();
    //display.set_orientation(&Orientation::Landscape).unwrap();

    // Create two frame buffers for double buffering
    let mut frame_buffer_1 = unsafe { FrameBuffer::new(&mut FRAME_BUFFER_1, LCD_WIDTH, LCD_HEIGHT) };
    let mut frame_buffer_2 = unsafe { FrameBuffer::new(&mut FRAME_BUFFER_2, LCD_WIDTH, LCD_HEIGHT) };


    //display.set_offset(1, 26);

    let lcd_zero = Point::zero();
    let lcd_max_corner = Point::new((LCD_WIDTH - 1) as i32, (LCD_HEIGHT - 1) as i32);

    let style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb565::BLUE)
        .build();

    //Please note that you must clear the screen before you turn on the backlight or else the screen will display static.
    display.clear(Rgb565::BLACK);
    delay.delay_ms(1000);
    // Set the backlight pin high
    _lcd_bl.set_high().unwrap();
    delay.delay_ms(1000);

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
        Point::new((LCD_WIDTH - 2) as i32, (LCD_HEIGHT - 2) as i32),
    )
    .into_styled(style)
    .draw(&mut display)
    .unwrap();

    delay.delay_ms(1000);

    Line::new(lcd_zero, lcd_max_corner)
        .into_styled(PrimitiveStyle::with_stroke(Rgb565::RED, 1))
        .draw(&mut display)
        .unwrap();

    Line::new(
        Point::new(0, (LCD_HEIGHT - 1) as i32),
        Point::new((LCD_WIDTH - 1) as i32, 0),
    )
    .into_styled(PrimitiveStyle::with_stroke(Rgb565::GREEN, 1))
    .draw(&mut display)
    .unwrap();
// Load image data
let image_data = include_bytes!("2wd-big-endian.raw");
    // Copy image data to the frame buffer
    //frame_buffer_1.get_mut_buffer()[..image_data.len()].copy_from_slice(image_data);
    //frame_buffer_2.get_mut_buffer()[..image_data.len()].copy_from_slice(image_data);


        // Draw image on the frame buffer
        let raw_image: ImageRaw<Rgb565> = ImageRaw::new(image_data, LCD_WIDTH);
        let image = Image::new(&raw_image, Point::zero());
        image.draw(&mut frame_buffer_1).unwrap();
        image.draw(&mut frame_buffer_2).unwrap();

        display.show(frame_buffer_1.get_buffer()).unwrap();

        delay.delay_ms(1000);

        let points = [
            Point::new(120, 100),
            Point::new(100, 140),
            Point::new(140, 140),
        ];

    // Draw a filled triangle on the frame buffers
    let style = PrimitiveStyleBuilder::new().fill_color(Rgb565::RED).build();
    Triangle::new(points[0], points[1], points[2])
        .into_styled(style)
        .draw(&mut frame_buffer_1)
        .unwrap();
    /* 
    Triangle::new(points[0], points[1], points[2])
        .into_styled(style)
        .draw(&mut frame_buffer_2)
        .unwrap();
    */
    // Show the initial buffer
    display.show(frame_buffer_1.get_buffer()).unwrap();
    delay.delay_ms(1000);

    // Draw the image to the display
    //display.draw_image(image_data).unwrap();

    // Draw a filled triangle on the frame buffers
    let style = PrimitiveStyleBuilder::new().fill_color(Rgb565::GREEN).build();
    Triangle::new(points[0], points[1], points[2])
        .into_styled(style)
        .draw(&mut frame_buffer_2)
        .unwrap();

        display.show(frame_buffer_2.get_buffer()).unwrap();
    delay.delay_ms(1000);
    

    //Reset the frame buffers.
    image.draw(&mut frame_buffer_1).unwrap();
    image.draw(&mut frame_buffer_2).unwrap();

        // Calculate the center of the image
        let arrow_rotate_point_x = 240 / 2;
        let arrow_rotate_point_y = (240 / 10) * 8;
    create_arrow_image_5(&mut frame_buffer_1, 45, arrow_rotate_point_x,arrow_rotate_point_y );
    create_arrow_image_5(&mut frame_buffer_2, 46, arrow_rotate_point_x,arrow_rotate_point_y );
    //frame_buffer_2.clear(Rgb565::BLACK);
    //image.draw(&mut frame_buffer_2).unwrap();
    //frame_buffer_2.get_mut_buffer()[..image_data.len()].copy_from_slice(image_data);
    
    // Infinite colour wheel loop
    let mut l: i32 = 0;
    let mut use_first_buffer = true;
    let mut c = Rgb565::RED;
    loop {
        // Alternate between buffers
        // Alternate between buffers
        let current_buffer = if use_first_buffer {
            frame_buffer_1.get_buffer()
        } else {
            frame_buffer_2.get_buffer()
        };


                // Clear the current buffer
                //current_buffer.clear(Rgb565::BLACK);
        // Call the show function with the buffer to display the image and arrow
        display.show(current_buffer).unwrap();
        //delay of 16 for 60Hz or 60 Frames a second.
        delay.delay_ms(1000);
        /* 
        Line::new(Point::new(0, l), Point::new((LCD_WIDTH - 1) as i32, l))
            .into_styled(PrimitiveStyle::with_stroke(c, 1))
            .draw(&mut display)
            .unwrap();
        delay.delay_ms(10);
        l += 1;
        if l == LCD_HEIGHT as i32 {
            l = 0;
            c = match c {
                Rgb565::RED => Rgb565::GREEN,
                Rgb565::GREEN => Rgb565::BLUE,
                _ => Rgb565::RED,
            }
        }
        */
        // Toggle the buffer
        use_first_buffer = !use_first_buffer;
    }

}

fn create_arrow_image_5(
    framebuffer: &mut FrameBuffer<>,
    angle: i32,
    compass_center_x: i32,
    compass_center_y: i32,
) {
    let compass_center = Point::new(compass_center_x, compass_center_y);
    let north_angle = angle - 180;
    let south_angle = angle;
    let north_left_angle = north_angle - 2;
    let north_right_angle = north_angle + 2;
    let south_left_angle = south_angle + 10;
    let south_right_angle = south_angle - 10;

    let circle_1 = 128;
    let circle_2 = 125;
    let circle_3 = 36;
    let circle_4 = 32;

    let north = get_coordinates(compass_center, circle_1, north_angle);
    let south = get_coordinates(compass_center, circle_4, south_angle);
    let north_left = get_coordinates(compass_center, circle_2, north_left_angle);
    let north_right = get_coordinates(compass_center, circle_2, north_right_angle);
    let south_left = get_coordinates(compass_center, circle_3, south_left_angle);
    let south_right = get_coordinates(compass_center, circle_3, south_right_angle);

    let merged_points = [
        north,
        north_left,
        south_left,
        south,
        south_right,
        north_right,
    ];

    let left_points = [
        north,
        north_left,
        south_left,
        south,
        Point::zero(),  // unused but needed to keep array size fixed
        Point::zero(),  // unused but needed to keep array size fixed
    ];

    let right_points = [
        north,
        north_right,
        south_right,
        south,
        Point::zero(),  // unused but needed to keep array size fixed
        Point::zero(),  // unused but needed to keep array size fixed
    ];

    let red = Rgb565::new(255, 0, 0);
    let red_9 = Rgb565::new(174, 31, 31);

    let style_red = PrimitiveStyleBuilder::new().fill_color(red).build();
    let style_red_9 = PrimitiveStyleBuilder::new().fill_color(red_9).build();

    draw_polygon(framebuffer, &merged_points, style_red_9);
    draw_polygon(framebuffer, &left_points[0..4], style_red);
    draw_polygon(framebuffer, &right_points[0..4], style_red_9);
}


fn draw_polygon(
    framebuffer: &mut FrameBuffer<>,
    points: &[Point],
    style: PrimitiveStyle<Rgb565>,
) {
    if points.len() < 3 {
        return; // Not enough points to form a polygon
    }

    // We will use a fan triangulation from the first point
    let first_point = points[0];

    for i in 1..points.len() - 1 {
        let triangle = Triangle::new(first_point, points[i], points[i + 1])
            .into_styled(style);
        triangle.draw(framebuffer).unwrap();
    }
}

// Helper function to calculate the coordinates based on the angle and radius
fn get_coordinates(center: Point, radius: i32, angle: i32) -> Point {
    let angle_rad = (angle as f32).to_radians() as f64;
    let x = center.x + (radius as f32 * cos(angle_rad) as f32) as i32;
    let y = center.y + (radius as f32 * sin(angle_rad) as f32) as i32;
    Point::new(x, y)
}