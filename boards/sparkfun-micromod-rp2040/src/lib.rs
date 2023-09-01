#![no_std]

//! Board Support Package for the SparkFun MicroMod RP2040.
//!
//! This crate serves as a HAL (Hardware Abstraction Layer) for the SparkFun MicroMod RP2040. Since the SparkFun MicroMod RP2040
//! is based on the RP2040 chip, it re-exports the [rp2040_hal] crate which contains the tooling to work with the
//! rp2040 chip.
//!
//! # Examples:
//!
//! The following example turns on the onboard LED. Note that most of the logic works through the [rp2040_hal] crate.
//! ```ignore
//! #![no_main]
//! use sparkfun_micromod_rp2040::entry;
//! use panic_halt as _;
//! use embedded_hal::digital::v2::OutputPin;
//! use sparkfun_micromod_rp2040::hal::pac;
//! use sparkfun_micromod_rp2040::hal;

//! #[entry]
//! fn does_not_have_to_be_main() -> ! {
//!   let mut pac = pac::Peripherals::take().unwrap();
//!   let sio = hal::Sio::new(pac.SIO);
//!   let pins = rp_pico::Pins::new(
//!        pac.IO_BANK0,
//!        pac.PADS_BANK0,
//!        sio.gpio_bank0,
//!        &mut pac.RESETS,
//!   );
//!   let mut led_pin = pins.led.into_push_pull_output();
//!   led_pin.set_high().unwrap();
//!   loop {
//!   }
//! }
//! ```

pub extern crate rp2040_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;

/// The `entry` macro declares the starting function to the linker.
/// This is similar to the `main` function in console applications.
///
/// It is based on the [cortex_m_rt](https://docs.rs/cortex-m-rt/latest/cortex_m_rt/attr.entry.html) crate.
///
/// # Examples
/// ```ignore
/// #![no_std]
/// #![no_main]
/// use sparkfun_micromod_rp2040::entry;
/// #[entry]
/// fn you_can_use_a_custom_main_name_here() -> ! {
///   loop {}
/// }
/// ```

#[cfg(feature = "rt")]
pub use hal::entry;

/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
#[cfg(feature = "boot2")]
#[link_section = ".boot2"]
#[no_mangle]
#[used]
pub static BOOT2_FIRMWARE: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

use hal::gpio::{bank0::Gpio29, FunctionSioInput, PullNone};
pub use hal::pac;

hal::bsp_pins!(
    /// GPIO 0 can serve as:
    /// - `UART_TX1`
    Gpio0 {
        name: gpio0,
        aliases: {
            FunctionUart, PullNone: UartTx1
        }
    },
    /// GPIO 1 can serve as:
    /// - `UART_RX1`
    Gpio1 {
        name: gpio1,
        aliases: {
            FunctionUart, PullNone: UartRx1
        }
    },
    /// GPIO 2 can serve as:
    /// - `UART_CTS1`
    /// - `AUD_LRCLK`
    Gpio2 {
        name: gpio2,
        aliases: {
            FunctionUart, PullNone: UartCts1,
            FunctionPio0, PullNone: AudLrclkPio0,
            FunctionPio1, PullNone: AudLrclkPio1
        }
    },
    /// GPIO 3 can serve as:
    /// - `UART_RTS1`
    /// - `AUD_BCLK`
    Gpio3 {
        name: gpio3,
        aliases: {
            FunctionUart, PullNone: UartRts1,
            FunctionPio0, PullNone: AudBclkPio0,
            FunctionPio1, PullNone: AudBclkPio1
        }
    },
    /// GPIO 4 can serve as:
    /// - `I2C_SDA`
    Gpio4 {
        name: gpio4,
        aliases: {
            FunctionI2C, PullUp: I2CSda
        }
    },
    /// GPIO 5 can serve as:
    /// - `I2C_SCL`
    Gpio5 {
        name: gpio5,
        aliases: {
            FunctionI2C, PullUp: I2CScl
        }
    },
    /// GPIO 6 can serve as:
    /// - `D0`
    Gpio6 {
        name: gpio6,
        aliases: {
            FunctionPwm, PullNone: D0Pwm,
            FunctionPio0, PullNone: D0Pio0,
            FunctionPio1, PullNone: D0Pio1
        }
    },
    /// GPIO 7 can serve as:
    /// - `D1`
    Gpio7 {
        name: gpio7,
        aliases: {
            FunctionPwm, PullNone: D1Pwm,
            FunctionPio0, PullNone: D1Pio0,
            FunctionPio1, PullNone: D1Pio1
        }
    },
    /// GPIO 8 can serve as:
    /// - `I2C_INT`
    /// - `UART_TX2`
    Gpio8 {
        name: gpio8,
        aliases: {
            FunctionI2C, PullUp: I2CInt,
            FunctionUart, PullNone: UartTx2
        }
    },
    /// GPIO 9 can serve as:
    /// - `SPI_CS1`
    /// - `UART_RX2`
    /// - `SDIO_DATA3`
    Gpio9 {
        name: gpio9,
        aliases: {
            FunctionUart, PullNone: UartRx2,
            FunctionSpi, PullNone: SpiCs1,
            FunctionPio0, PullNone: SdioData3Pio0,
            FunctionPio1, PullNone: SdioData3Pio1
        }
    },
    /// GPIO 10 can serve as:
    /// - `SDIO_DATA2`
    /// - `AUD_OUT`
    Gpio10 {
        name: gpio10,
        aliases: {
            FunctionPio0, PullNone: SdioData2Pio0,
            FunctionPio1, PullNone: SdioData2Pio1,
            FunctionPio0, PullNone: AudOutPio0,
            FunctionPio1, PullNone: AudOutPio1
        }
    },
    /// GPIO 11 can serve as:
    /// - `SDIO_DATA1`
    /// - `AUD_IN`
    Gpio11 {
        name: gpio11,
        aliases: {
            FunctionPio0, PullNone: SdioData1Pio0,
            FunctionPio1, PullNone: SdioData1Pio1,
            FunctionPio0, PullNone: AudInPio0,
            FunctionPio1, PullNone: AudInPio1
        }
    },
    /// GPIO 12 can serve as:
    /// - `SPI_COPI1`
    /// - `SDIO_DATA0`
    Gpio12 {
        name: gpio12,
        aliases: {
            FunctionSpi, PullNone: SpiCipo1,
            FunctionPio0, PullNone: SdioData0Pio0,
            FunctionPio1, PullNone: SdioData0Pio1
        }
    },
    /// GPIO 13 can serve as:
    /// - `PWM0`
    Gpio13 {
        name: gpio13,
        aliases: {
            FunctionPwm, PullNone: Pwm0
        }
    },
    /// GPIO 14 can serve as:
    /// - `SPI_SCK1`
    /// - `SDIO_SCK`
    Gpio14 {
        name: gpio14,
        aliases: {
            FunctionSpi, PullNone: SpiSck1,
            FunctionPio0, PullNone: SdioSckPio0,
            FunctionPio1, PullNone: SdioSckPio1
        }
    },
    /// GPIO 15 can serve as:
    /// - `SPI_COPI1`
    /// - `SDIO_CMD`
    Gpio15 {
        name: gpio15,
        aliases: {
            FunctionSpi, PullNone: SpiCopi1,
            FunctionPio0, PullNone: SdioCmdPio0,
            FunctionPio1, PullNone: SdioCmdPio1
        }
    },
    /// GPIO 16 can serve as:
    /// - `G0`
    Gpio16 {
        name: gpio16,
        aliases: {
            FunctionPwm, PullNone: G0Pwm,
            FunctionPio0, PullNone: G0Pio0,
            FunctionPio1, PullNone: G0Pio1
        }
    },
    /// GPIO 17 can serve as:
    /// - `G1`
    Gpio17 {
        name: gpio17,
        aliases: {
            FunctionPwm, PullNone: G1Pwm,
            FunctionPio0, PullNone: G1Pio0,
            FunctionPio1, PullNone: G1Pio1
        }
    },
    /// GPIO 18 can serve as:
    /// - `G2`
    Gpio18 {
        name: gpio18,
        aliases: {
            FunctionPwm, PullNone: G2Pwm,
            FunctionPio0, PullNone: G2Pio0,
            FunctionPio1, PullNone: G2Pio1
        }
    },
    /// GPIO 19 can serve as:
    /// - `G3`
    Gpio19 {
        name: gpio19,
        aliases: {
            FunctionPwm, PullNone: G3Pwm,
            FunctionPio0, PullNone: G3Pio0,
            FunctionPio1, PullNone: G3Pio1
        }
    },
    /// GPIO 20 can serve as:
    /// - `SPI_CIPO`
    /// - `G4`
    Gpio20 {
        name: gpio20,
        aliases: {
            FunctionSpi, PullNone: SpiCipo,
            FunctionPwm, PullNone: G4Pwm,
            FunctionPio0, PullNone: G4Pio0,
            FunctionPio1, PullNone: G4Pio1
        }
    },
    /// GPIO 21 can serve as:
    /// - `SPI_CS`
    /// - `G5`
    Gpio21 {
        name: gpio21,
        aliases: {
            FunctionSpi, PullNone: SpiCs,
            FunctionPwm, PullNone: G5Pwm,
            FunctionPio0, PullNone: G5Pio0,
            FunctionPio1, PullNone: G5Pio1
        }
    },
    /// GPIO 22 can serve as:
    /// - `SPI_SCK`
    /// - `G6`
    Gpio22 {
        name: gpio22,
        aliases: {
            FunctionSpi, PullNone: SpiSck,
            FunctionPwm, PullNone: G6Pwm,
            FunctionPio0, PullNone: G6Pio0,
            FunctionPio1, PullNone: G6Pio1
        }
    },
    /// GPIO 23 can serve as:
    /// - `SPI_COPI`
    /// - `G7`
    Gpio23 {
        name: gpio23,
        aliases: {
            FunctionSpi, PullNone: SpiCopi,
            FunctionPwm, PullNone: G7Pwm,
            FunctionPio0, PullNone: G7Pio0,
            FunctionPio1, PullNone: G7Pio1
        }
    },
    /// GPIO 24 can serve as:
    /// - `PWM1`
    /// - `AUD_MCLK`
    Gpio24 {
        name: gpio24,
        aliases: {
            FunctionPwm, PullNone: Pwm1,
            FunctionPio0, PullNone: AudMclkPio0,
            FunctionPio1, PullNone: AudMclkPio1
        }
    },
    /// GPIO 25 can serve as:
    /// - Builtin LED
    /// - `G10`
    Gpio25 {
        name: led,
        aliases: {
            FunctionPwm, PullNone: G10Pwm,
            FunctionPio0, PullNone: G10Pio0,
            FunctionPio1, PullNone: G10Pio1
        }
    },
    /// ADC 0 can serve as:
    /// - `ADC0`
    Gpio26 {
        name: adc0,
    },
    /// ADC 1 can serve as:
    /// - `ADC1`
    Gpio27 {
        name: adc1,
    },
    /// GPIO 28 can serve as:
    /// - `G9`
    Gpio28 {
        name: gpio28,
        aliases: {
            FunctionPwm, PullNone: G9Pwm,
            FunctionPio0, PullNone: G9Pio0,
            FunctionPio1, PullNone: G9Pio1
        }
    },
    /// ADC 3 can serve as:
    /// - `BATT_VIN`
    Gpio29 {
        name: batt_vin,
    },
);

pub const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;

/// Alias for a configured pin
pub type BattVin = hal::adc::AdcPin<hal::gpio::Pin<Gpio29, FunctionSioInput, PullNone>>;
/// Driver for reading the battery volatage
pub struct BatteryVoltage {
    pin: BattVin,
}

impl BatteryVoltage {
    /// Creates a new battery voltage reader
    pub fn new(pin: BattVin) -> Self {
        Self { pin }
    }

    /// Reads the current battery voltage
    ///
    /// # Return
    ///
    /// The current voltage in millivolts
    pub fn read(&mut self, adc: &mut hal::Adc) -> u16 {
        use embedded_hal::adc::OneShot;

        let raw_value: u32 = loop {
            match adc.read(&mut self.pin) {
                Ok(val) => break val,
                Err(nb::Error::WouldBlock) => (),
                Err(nb::Error::Other(_)) => unreachable!(),
            }
        };

        // Convert value to millivolts
        // The raw ADC value is in in the range of 0..4096,
        // where 0 = 0V and 4096 = 3.3V.
        // The MicroMod interface defines that the voltage is divided by 3,
        // so the conversion formula:
        // value / 4096 * 3300(mV) * 3
        let value = (raw_value * 3300 * 3) / 4096;

        // The maximum possible value is 9900, so it's safe to convert
        // back to u16.
        value as u16
    }
}
