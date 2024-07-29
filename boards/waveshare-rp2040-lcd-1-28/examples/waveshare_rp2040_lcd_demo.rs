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
    primitives::{PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, Triangle, Circle},
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
    let lcd_dc = pins.gp8.into_push_pull_output();
    let lcd_cs = pins.gp9.into_push_pull_output();
    let lcd_clk = pins.gp10.into_function::<hal::gpio::FunctionSpi>();
    let lcd_mosi = pins.gp11.into_function::<hal::gpio::FunctionSpi>();
    let lcd_rst = pins.gp12.into_push_pull_output_in_state(hal::gpio::PinState::High);
    let mut _lcd_bl = pins.gp25.into_push_pull_output_in_state(hal::gpio::PinState::Low);
    
    // Initialize SPI
    let spi = hal::Spi::<_, _, _, 8>::new(pac.SPI1, (lcd_mosi, lcd_clk));
    let spi = spi.init(
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
        40.MHz(),
        embedded_hal::spi::MODE_0,
    );

    // Initialize the display
    let mut display = GC9A01A::new(spi, lcd_dc, lcd_cs, lcd_rst, false, true, LCD_WIDTH, LCD_HEIGHT);
    display.init(&mut delay).unwrap();

    // Create two frame buffers for double buffering
    let mut frame_buffer_1 = unsafe { FrameBuffer::new(&mut FRAME_BUFFER_1, LCD_WIDTH, LCD_HEIGHT) };
    let mut frame_buffer_2 = unsafe { FrameBuffer::new(&mut FRAME_BUFFER_2, LCD_WIDTH, LCD_HEIGHT) };

    // Define LCD dimensions
    let lcd_zero = Point::zero();
    let lcd_max_corner = Point::new((LCD_WIDTH - 1) as i32, (LCD_HEIGHT - 1) as i32);

    // Define a style for the rectangle
    let style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb565::BLUE)
        .build();

    // Clear the screen before turning on the backlight
    display.clear(Rgb565::BLACK);
    delay.delay_ms(1000);
    _lcd_bl.set_high().unwrap();
    delay.delay_ms(1000);

    // Draw a blue rectangle
    Rectangle::with_corners(lcd_zero, lcd_max_corner)
        .into_styled(style)
        .draw(&mut display)
        .unwrap();
    delay.delay_ms(1000);

    // Draw a black rectangle inside the blue rectangle
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

    // Draw red and green lines
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
    //let image_data = include_bytes!("2wd-big-endian.raw");
    let image_data = include_bytes!("fuel-big-endian.raw");
    
    let raw_image: ImageRaw<Rgb565> = ImageRaw::new(image_data, LCD_WIDTH);
    let image = Image::new(&raw_image, Point::zero());

    // Draw the image on both frame buffers
    image.draw(&mut frame_buffer_1).unwrap();
    image.draw(&mut frame_buffer_2).unwrap();
    display.show(frame_buffer_1.get_buffer()).unwrap();
    delay.delay_ms(1000);

    // Draw a filled red triangle on the first frame buffer
    let points = [
        Point::new(120, 100),
        Point::new(100, 140),
        Point::new(140, 140),
    ];
    let style = PrimitiveStyleBuilder::new().fill_color(Rgb565::RED).build();
    Triangle::new(points[0], points[1], points[2])
        .into_styled(style)
        .draw(&mut frame_buffer_1)
        .unwrap();
    display.show(frame_buffer_1.get_buffer()).unwrap();
    delay.delay_ms(1000);

    // Draw a filled green triangle on the second frame buffer
    let style = PrimitiveStyleBuilder::new().fill_color(Rgb565::GREEN).build();
    Triangle::new(points[0], points[1], points[2])
        .into_styled(style)
        .draw(&mut frame_buffer_2)
        .unwrap();
    display.show(frame_buffer_2.get_buffer()).unwrap();
    delay.delay_ms(1000);
    
    // Reset the frame buffers
    image.draw(&mut frame_buffer_1).unwrap();
    image.draw(&mut frame_buffer_2).unwrap();

    // Calculate the center of the image
    let arrow_rotate_point_x = 240 / 2;
    let arrow_rotate_point_y = (240 / 10) * 8;
    create_arrow_image_5(&mut frame_buffer_1, 45, arrow_rotate_point_x, arrow_rotate_point_y);
    create_arrow_image_5(&mut frame_buffer_2, 46, arrow_rotate_point_x, arrow_rotate_point_y);

    let compass_center = Point::new(240 / 2, (240 / 10) * 8);
    let arrow_points = precompute_arrow_points(compass_center, 45, 135);
    let mut increasing = true;
    let mut bounding_box : Rectangle; 
    let mut previous_bounding_box = Rectangle::new(Point::new(0, 0), Size::new(0, 0));
        // Define a rectangle at (0, 0) with width 0 and height 0
    let mut angle_index: usize = 0;

    //Create a background buffer with frame_buffer_1
    frame_buffer_1.get_mut_buffer()[..image_data.len()].copy_from_slice(image_data);
    //Create a drawing to lcd buffer with frame_buffer_2
    frame_buffer_2.get_mut_buffer()[..image_data.len()].copy_from_slice(image_data);
    //Show the display to present the initial image.
    display.show(frame_buffer_2.get_buffer()).unwrap();

    loop {
        let start_time = cortex_m::peripheral::SYST::get_current();
        let points = &arrow_points[angle_index];

        //Copy the previous bounding box from the background buffer (frame_buffer_1) into the lcd buffer (frame_buffer_2)
        //This prevents the whole reload of the image_data.
        let previous_bounding_box_buffer = &frame_buffer_1.get_buffer()[(previous_bounding_box.top_left.y as usize * LCD_WIDTH as usize * 2) + (previous_bounding_box.top_left.x as usize * 2)..];
        let destination_buffer = &mut frame_buffer_2.get_mut_buffer()[(previous_bounding_box.top_left.y as usize * LCD_WIDTH as usize * 2) + (previous_bounding_box.top_left.x as usize * 2)..];

        for row in 0..previous_bounding_box.size.height as usize {
            let source_row_start = row * LCD_WIDTH as usize * 2;
            let source_row_end = source_row_start + previous_bounding_box.size.width as usize * 2;
            destination_buffer[source_row_start..source_row_end].copy_from_slice(&previous_bounding_box_buffer[source_row_start..source_row_end]);
        }

        //Draw the arrow and return the new bounding box
        bounding_box = create_arrow_image_6(&mut frame_buffer_2, points);
        //Draw the center button
        create_button_image_1(&mut frame_buffer_2, arrow_rotate_point_x, arrow_rotate_point_y);

        // Adjust the angle index for the next iteration
        if increasing {
            angle_index += 1;
            if angle_index >= arrow_points.len() - 1{
                increasing = false;
            }
        } else {
            angle_index -= 1;
            if angle_index <= 0 {
                increasing = true;
            }
        }

        //the bounding box has a pixel padding of 5 pixels around the arrow to prevent the need to draw the background buffer before the next arrow is drawn.  
        //This improves performance as only one draw operation occurs instead of 2.
        display.show_region_2(frame_buffer_2.get_buffer(), bounding_box).unwrap();
        previous_bounding_box = bounding_box;

        // Calculate the frame time and adjust delay to achieve ~60 FPS
        let frame_time = cortex_m::peripheral::SYST::get_current().wrapping_sub(start_time);
        let frame_time_ms = (frame_time as f64) / (sys_freq as f64 / 1_000.0);
        let target_frame_time_ms = 16.67;
        if frame_time_ms < target_frame_time_ms {
            //delay.delay_ms((target_frame_time_ms - frame_time_ms) as u32);
        }
    }
}

fn create_arrow_image_5(
    framebuffer: &mut FrameBuffer,
    angle: i32,
    compass_center_x: i32,
    compass_center_y: i32,
) -> Rectangle{
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
    let red_9 = Rgb565::new(19,1,1);

    let style_red = PrimitiveStyleBuilder::new().fill_color(red).build();
    let style_red_9 = PrimitiveStyleBuilder::new().fill_color(red_9).build();

    draw_polygon(framebuffer, &merged_points, style_red_9);
    draw_polygon(framebuffer, &left_points[0..4], style_red);
    draw_polygon(framebuffer, &right_points[0..4], style_red_9);

        // Calculate the bounding box of the arrow
        let bounding_box = calculate_bounding_box(&merged_points, Some(5));

        bounding_box
}

fn draw_polygon(
    framebuffer: &mut FrameBuffer,
    points: &[Point],
    style: PrimitiveStyle<Rgb565>,
) {
    if points.len() < 3 {
        return; // Not enough points to form a polygon
    }

    // Use fan triangulation from the first point
    let first_point = points[0];
    for i in 1..points.len() - 1 {
        let triangle = Triangle::new(first_point, points[i], points[i + 1])
            .into_styled(style);
        triangle.draw(framebuffer).unwrap();
    }
}

// Helper function to calculate coordinates based on angle and radius
fn get_coordinates(center: Point, radius: i32, angle: i32) -> Point {
    let angle_rad = (angle as f32).to_radians() as f64;
    let x = center.x + (radius as f32 * cos(angle_rad) as f32) as i32;
    let y = center.y + (radius as f32 * sin(angle_rad) as f32) as i32;
    Point::new(x, y)
}
/// Converts RGB888 color to RGB565 format.
fn convert_rgb888_to_color565(r: u8, g: u8, b: u8, big_endian: bool) -> Rgb565 {
    let val16 = ((r & 0xf8) as u16) << 8 | ((g & 0xfc) as u16) << 3 | (b >> 3) as u16;
    let value = if big_endian {
        val16.swap_bytes()
    } else {
        val16
    };

    Rgb565::new((value >> 11) as u8 & 0x1f, (value >> 5) as u8 & 0x3f, (value & 0x1f) as u8)
}

/// Draws a circle on the frame buffer.
fn draw_circle(framebuffer: &mut FrameBuffer, color: Rgb565, center: Point, radius: i32) {
    let style = PrimitiveStyleBuilder::new()
        .fill_color(color)
        .build();
        // Calculate the top-left corner of the circle based on the center point and radius
    let top_left = Point::new(center.x - radius, center.y - radius);
    let diameter = (radius * 2) as u32;

    Circle::new(top_left, diameter as u32)
        .into_styled(style)
        .draw(framebuffer)
        .unwrap();
}
/// Creates a button image on the frame buffer.
fn create_button_image_1(framebuffer: &mut FrameBuffer, center_x: i32, center_y: i32) {
    let button_color_top = Rgb565::new(8, 16, 8);
    let button_color_shadow = Rgb565::new(12, 23, 12);

    let circle_radius = 14;
    draw_circle(framebuffer, button_color_shadow, Point::new(center_x - 1, center_y - 1), circle_radius);
    draw_circle(framebuffer, button_color_top, Point::new(center_x, center_y), circle_radius);
}

/// Helper function to calculate the bounding box of a set of points with an optional padding.
fn calculate_bounding_box(points: &[Point], padding: Option<u32>) -> Rectangle {
    let mut min_x = points[0].x;
    let mut max_x = points[0].x;
    let mut min_y = points[0].y;
    let mut max_y = points[0].y;

    for point in points.iter().skip(1) {
        if point.x < min_x {
            min_x = point.x;
        }
        if point.x > max_x {
            max_x = point.x;
        }
        if point.y < min_y {
            min_y = point.y;
        }
        if point.y > max_y {
            max_y = point.y;
        }
    }

    let padding = padding.unwrap_or(0) as i32;
    Rectangle::new(
        Point::new(min_x - padding, min_y - padding),
        Size::new((max_x - min_x + 2 * padding) as u32, (max_y - min_y + 2 * padding) as u32),
    )
}

#[derive(Copy, Clone)]
struct ArrowPoints {
    north: Point,
    south: Point,
    north_left: Point,
    north_right: Point,
    south_left: Point,
    south_right: Point,
}

fn precompute_arrow_points(
    compass_center: Point,
    start_angle: i32,
    end_angle: i32,
) -> [ArrowPoints; 91] {
    let mut points_array = [ArrowPoints {
        north: Point::zero(),
        south: Point::zero(),
        north_left: Point::zero(),
        north_right: Point::zero(),
        south_left: Point::zero(),
        south_right: Point::zero(),
    }; 91];

    for angle in start_angle..=end_angle {
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

        points_array[(angle - start_angle) as usize] = ArrowPoints {
            north,
            south,
            north_left,
            north_right,
            south_left,
            south_right,
        };
    }

    points_array
}

fn create_arrow_image_6(
    framebuffer: &mut FrameBuffer,
    points: &ArrowPoints,
) -> Rectangle {
    let merged_points = [
        points.north,
        points.north_left,
        points.south_left,
        points.south,
        points.south_right,
        points.north_right,
    ];

    let left_points = [
        points.north,
        points.north_left,
        points.south_left,
        points.south,
        Point::zero(), // unused but needed to keep array size fixed
        Point::zero(), // unused but needed to keep array size fixed
    ];

    let right_points = [
        points.north,
        points.north_right,
        points.south_right,
        points.south,
        Point::zero(), // unused but needed to keep array size fixed
        Point::zero(), // unused but needed to keep array size fixed
    ];

    let red = Rgb565::new(255, 0, 0);
    let red_9 = Rgb565::new(19, 1, 1);

    let style_red = PrimitiveStyleBuilder::new().fill_color(red).build();
    let style_red_9 = PrimitiveStyleBuilder::new().fill_color(red_9).build();

    draw_polygon(framebuffer, &merged_points, style_red_9);
    draw_polygon(framebuffer, &left_points[0..4], style_red);
    draw_polygon(framebuffer, &right_points[0..4], style_red_9);

    // Calculate the bounding box of the arrow
    let bounding_box = calculate_bounding_box(&merged_points, Some(5));

    bounding_box
}