//! Using an Adafruit CANBus Feather RP2040 board, primary LED blinks and CAN
//! transmits once per second. LED goes solid upon receipt of any CAN message.
//!
#![no_std]
#![no_main]

use adafruit_canbus_feather_rp2040::entry;
use adafruit_canbus_feather_rp2040::{
    hal::{
        clocks::{init_clocks_and_plls, Clock},
        pac,
        spi,
        gpio,
        watchdog::Watchdog,
        Sio,
    },
    Pins, XOSC_CRYSTAL_FREQ,
};
use panic_halt as _;
use fugit::RateExtU32;
use mcp25xx::bitrates::clock_16mhz::CNF_500K_BPS;
use mcp25xx::registers::{OperationMode, RXB0CTRL, RXM};
use mcp25xx::{CanFrame, Config, MCP25xx};
use embedded_can::{Frame, StandardId};
use embedded_can::nb::Can;
use embedded_hal::digital::OutputPin;
use embedded_hal_bus::spi::{ExclusiveDevice, NoDelay};

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

    let spi_sclk: gpio::Pin<_, gpio::FunctionSpi, gpio::PullNone> = pins.sclk.reconfigure();
    let spi_mosi: gpio::Pin<_, gpio::FunctionSpi, gpio::PullNone> = pins.mosi.reconfigure();
    let spi_miso: gpio::Pin<_, gpio::FunctionSpi, gpio::PullNone> = pins.miso.reconfigure();
    let spi_cs = pins.can_cs.into_push_pull_output();

    let spi = spi::Spi::<_, _, _, 8>::new(pac.SPI1, (spi_mosi, spi_miso, spi_sclk));
    let spi = spi.init(
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
        400.kHz(),
        embedded_hal::spi::MODE_0,
    );

    // pins.can_standby
    // pins.can_tx0_rtx
    // pins.can_reset
    // pins.can_rx0_bf

    let spi_wrapper = ExclusiveDevice::new(spi, spi_cs, NoDelay).unwrap();
    let mut mcp25xx = MCP25xx { spi: spi_wrapper };

    let config = Config::default()
        .mode(OperationMode::NormalOperation)
        .bitrate(CNF_500K_BPS)
        .receive_buffer_0(RXB0CTRL::default().with_rxm(RXM::ReceiveAny));

    let _ = mcp25xx.apply_config(&config);

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let mut led_pin = pins.d13.into_push_pull_output();

    let can_id = StandardId::new(123).unwrap();
    let data = [1, 2, 3, 4, 5, 6, 7, 8];
    let frame = CanFrame::new(can_id, &data).unwrap();

    let wait_ms = 1000;
    let mut received = false;

    loop {
        led_pin.set_high().unwrap();
        delay.delay_ms(wait_ms);

        mcp25xx.transmit(&frame).unwrap();

        if let Ok(_) = mcp25xx.receive() {
            received = true;
        }

        if !received {
            led_pin.set_low().unwrap();
        }
        delay.delay_ms(wait_ms);

    }
}
