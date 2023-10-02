//! # Pico SD Card Example
//!
//! Reads and writes a file from/to the SD Card that is formatted in FAT32.
//! This example uses the SPI0 device of the Raspberry Pi Pico on the
//! pins 4,5,6 and 7. If you don't use an external 3.3V power source,
//! you can connect the +3.3V output on pin 36 to the SD card.
//!
//! SD Cards up to 2TB are supported by the `embedded_sdmmc` crate.
//! I've tested this with a 64GB micro SD card.
//!
//! You need to format the card with an regular old FAT32 filesystem
//! and also make sure the first partition has the right type. This is how your
//! `fdisk` output should look like:
//!
//! ```text
//!     fdisk /dev/sdj
//!
//!     Welcome to fdisk (util-linux 2.34).
//!     Changes will remain in memory only, until you decide to write them.
//!     Be careful before using the write command.
//!
//!     Command (m for help): Disk /dev/sdj:
//!     59,49 GiB, 63864569856 bytes, 124735488 sectors
//!     Disk model: SD/MMC/MS/MSPRO
//!     Units: sectors of 1 * 512 = 512 bytes
//!     Sector size (logical/physical): 512 bytes / 512 bytes
//!     I/O size (minimum/optimal): 512 bytes / 512 bytes
//!     Disklabel type: dos
//!     Disk identifier: 0x00000000
//!
//!     Device     Boot Start       End   Sectors  Size Id Type
//!     /dev/sdj1        2048 124735487 124733440 59,5G  c W95 FAT32 (LBA)
//! ```
//!
//! The important bit here is the _Type_ with `W95 FAT32 (LBA)`, other types
//! are rejected by the `embedded_sdmmc` filesystem implementation.
//!
//! Formatting the partition can be done using `mkfs.fat`:
//!
//!     $ mkfs.fat /dev/sdj1
//!
//! The example can either be used with a probe to receive debug output
//! and also the LED is used as status output. There are different blinking
//! patterns.
//!
//! For every successful stage in the example the LED will blink long once.
//! If everything is successful (9 long blink signals), the example will go
//! into a loop and either blink in a _"short long"_ or _"short short long"_ pattern.
//!
//! If there are 4 different error patterns, all with short blinking pulses:
//!
//! - **3 short blink (in a loop)**: Card size could not be retrieved.
//! - **4 short blink (in a loop)**: Error getting volume/partition 0.
//! - **5 short blink (in a loop)**: Error opening root directory.
//! - **6 short blink (in a loop)**: Could not open file 'O.TST'.
//!
//! See the `Cargo.toml` file for Copyright and license details.

#![no_std]
#![no_main]

use core::cell::RefCell;

// The macro for our start-up function
use rp_pico::entry;

// info!() and error!() macros for printing information to the debug output
use defmt::*;
use defmt_rtt as _;

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// Pull in any important traits
use rp_pico::hal::prelude::*;

// Embed the `Hz` function/trait:
use fugit::RateExtU32;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use rp_pico::hal::pac;

// Import the SPI abstraction:
use rp_pico::hal::spi;

// Import the GPIO abstraction:
use rp_pico::hal::gpio;

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use rp_pico::hal;

// Link in the embedded_sdmmc crate.
// The `SdMmcSpi` is used for block level access to the card.
// And the `VolumeManager` gives access to the FAT filesystem functions.
use embedded_sdmmc::{SdCard, TimeSource, Timestamp, VolumeIdx, VolumeManager};

// Get the file open mode enum:
use embedded_sdmmc::filesystem::Mode;

use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::blocking::delay::DelayUs;

/// A dummy timesource, which is mostly important for creating files.
#[derive(Default)]
pub struct DummyTimesource();

impl TimeSource for DummyTimesource {
    // In theory you could use the RTC of the rp2040 here, if you had
    // any external time synchronizing device.
    fn get_timestamp(&self) -> Timestamp {
        Timestamp {
            year_since_1970: 0,
            zero_indexed_month: 0,
            zero_indexed_day: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }
}

// Setup some blinking codes:
const BLINK_OK_LONG: [u8; 1] = [8u8];
const BLINK_OK_SHORT_LONG: [u8; 4] = [1u8, 0u8, 6u8, 0u8];
const BLINK_OK_SHORT_SHORT_LONG: [u8; 6] = [1u8, 0u8, 1u8, 0u8, 6u8, 0u8];
const BLINK_ERR_3_SHORT: [u8; 6] = [1u8, 0u8, 1u8, 0u8, 1u8, 0u8];
const BLINK_ERR_4_SHORT: [u8; 8] = [1u8, 0u8, 1u8, 0u8, 1u8, 0u8, 1u8, 0u8];
const BLINK_ERR_5_SHORT: [u8; 10] = [1u8, 0u8, 1u8, 0u8, 1u8, 0u8, 1u8, 0u8, 1u8, 0u8];
const BLINK_ERR_6_SHORT: [u8; 12] = [1u8, 0u8, 1u8, 0u8, 1u8, 0u8, 1u8, 0u8, 1u8, 0u8, 1u8, 0u8];

fn blink_signals(
    pin: &mut dyn embedded_hal::digital::v2::OutputPin<Error = core::convert::Infallible>,
    delay: &mut dyn DelayMs<u32>,
    sig: &[u8],
) {
    for bit in sig {
        if *bit != 0 {
            pin.set_high().unwrap();
        } else {
            pin.set_low().unwrap();
        }

        let length = if *bit > 0 { *bit } else { 1 };

        for _ in 0..length {
            delay.delay_ms(100);
        }
    }

    pin.set_low().unwrap();

    delay.delay_ms(500);
}

fn blink_signals_loop(
    pin: &mut dyn embedded_hal::digital::v2::OutputPin<Error = core::convert::Infallible>,
    delay: &mut dyn DelayMs<u32>,
    sig: &[u8],
) -> ! {
    loop {
        blink_signals(pin, delay, sig);
        delay.delay_ms(1000);
    }
}

#[entry]
fn main() -> ! {
    info!("Program start");

    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
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
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Set the LED to be an output
    let mut led_pin = pins.led.into_push_pull_output();

    // Set up our SPI pins into the correct mode
    let spi_sclk: gpio::Pin<_, gpio::FunctionSpi, gpio::PullNone> = pins.gpio2.reconfigure();
    let spi_mosi: gpio::Pin<_, gpio::FunctionSpi, gpio::PullNone> = pins.gpio3.reconfigure();
    let spi_miso: gpio::Pin<_, gpio::FunctionSpi, gpio::PullUp> = pins.gpio4.reconfigure();
    let spi_cs = pins.gpio5.into_push_pull_output();

    // Create the SPI driver instance for the SPI0 device
    let spi = spi::Spi::<_, _, _, 8>::new(pac.SPI0, (spi_mosi, spi_miso, spi_sclk));

    // Exchange the uninitialised SPI driver for an initialised one
    let spi = spi.init(
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
        400.kHz(), // card initialization happens at low baud rate
        embedded_hal::spi::MODE_0,
    );

    // We need a delay implementation that can be passed to SdCard and still be used
    // for the blink signals.
    let mut delay = &SharedDelay::new(cortex_m::delay::Delay::new(
        core.SYST,
        clocks.system_clock.freq().to_Hz(),
    ));

    info!("Initialize SPI SD/MMC data structures...");
    let sdcard = SdCard::new(spi, spi_cs, delay);
    let mut volume_mgr = VolumeManager::new(sdcard, DummyTimesource::default());

    blink_signals(&mut led_pin, &mut delay, &BLINK_OK_LONG);

    info!("Init SD card controller and retrieve card size...");
    match volume_mgr.device().num_bytes() {
        Ok(size) => info!("card size is {} bytes", size),
        Err(e) => {
            error!("Error retrieving card size: {}", defmt::Debug2Format(&e));
            blink_signals_loop(&mut led_pin, &mut delay, &BLINK_ERR_3_SHORT);
        }
    }

    blink_signals(&mut led_pin, &mut delay, &BLINK_OK_LONG);

    // Now that the card is initialized, clock can go faster
    volume_mgr
        .device()
        .spi(|spi| spi.set_baudrate(clocks.peripheral_clock.freq(), 16.MHz()));

    info!("Getting Volume 0...");
    let mut volume = match volume_mgr.get_volume(VolumeIdx(0)) {
        Ok(v) => v,
        Err(e) => {
            error!("Error getting volume 0: {}", defmt::Debug2Format(&e));
            blink_signals_loop(&mut led_pin, &mut delay, &BLINK_ERR_4_SHORT);
        }
    };

    blink_signals(&mut led_pin, &mut delay, &BLINK_OK_LONG);

    // After we have the volume (partition) of the drive we got to open the
    // root directory:
    let dir = match volume_mgr.open_root_dir(&volume) {
        Ok(dir) => dir,
        Err(e) => {
            error!("Error opening root dir: {}", defmt::Debug2Format(&e));
            blink_signals_loop(&mut led_pin, &mut delay, &BLINK_ERR_5_SHORT);
        }
    };

    info!("Root directory opened!");
    blink_signals(&mut led_pin, &mut delay, &BLINK_OK_LONG);

    // This shows how to iterate through the directory and how
    // to get the file names (and print them in hope they are UTF-8 compatible):
    volume_mgr
        .iterate_dir(&volume, &dir, |ent| {
            info!(
                "/{}.{}",
                core::str::from_utf8(ent.name.base_name()).unwrap(),
                core::str::from_utf8(ent.name.extension()).unwrap()
            );
        })
        .unwrap();

    blink_signals(&mut led_pin, &mut delay, &BLINK_OK_LONG);

    let mut successful_read = false;

    // Next we going to read a file from the SD card:
    if let Ok(mut file) = volume_mgr.open_file_in_dir(&mut volume, &dir, "O.TST", Mode::ReadOnly) {
        let mut buf = [0u8; 32];
        let read_count = volume_mgr.read(&volume, &mut file, &mut buf).unwrap();
        volume_mgr.close_file(&volume, file).unwrap();

        if read_count >= 2 {
            info!("READ {} bytes: {}", read_count, buf);

            // If we read what we wrote before the last reset,
            // we set a flag so that the success blinking at the end
            // changes it's pattern.
            if buf[0] == 0x42 && buf[1] == 0x1E {
                successful_read = true;
            }
        }
    }

    blink_signals(&mut led_pin, &mut delay, &BLINK_OK_LONG);

    match volume_mgr.open_file_in_dir(&mut volume, &dir, "O.TST", Mode::ReadWriteCreateOrTruncate) {
        Ok(mut file) => {
            volume_mgr
                .write(&mut volume, &mut file, b"\x42\x1E")
                .unwrap();
            volume_mgr.close_file(&volume, file).unwrap();
        }
        Err(e) => {
            error!("Error opening file 'O.TST': {}", defmt::Debug2Format(&e));
            blink_signals_loop(&mut led_pin, &mut delay, &BLINK_ERR_6_SHORT);
        }
    }

    volume_mgr.free();

    blink_signals(&mut led_pin, &mut delay, &BLINK_OK_LONG);

    if successful_read {
        info!("Successfully read previously written file 'O.TST'");
    } else {
        info!("Could not read file, which is ok for the first run.");
        info!("Reboot the pico!");
    }

    loop {
        if successful_read {
            blink_signals(&mut led_pin, &mut delay, &BLINK_OK_SHORT_SHORT_LONG);
        } else {
            blink_signals(&mut led_pin, &mut delay, &BLINK_OK_SHORT_LONG);
        }

        delay.delay_ms(1000);
    }
}

// Can be removed once we have https://github.com/rp-rs/rp-hal/pull/614,
// ie. when we move to rp2040-hal 0.9
struct SharedDelay {
    inner: RefCell<cortex_m::delay::Delay>,
}

impl SharedDelay {
    fn new(delay: cortex_m::delay::Delay) -> Self {
        Self {
            inner: delay.into(),
        }
    }
}

impl DelayMs<u32> for &SharedDelay {
    fn delay_ms(&mut self, ms: u32) {
        self.inner.borrow_mut().delay_ms(ms);
    }
}

impl DelayUs<u8> for &SharedDelay {
    fn delay_us(&mut self, us: u8) {
        self.inner.borrow_mut().delay_us(us as u32);
    }
}
