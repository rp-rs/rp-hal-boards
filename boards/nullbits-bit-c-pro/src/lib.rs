#![no_std]

pub extern crate rp2040_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use hal::entry;

// BIT-C uses BY25Q32BSTIG flash chip. Should work with BOOT_LOADER_W25X10CL

/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
#[cfg(feature = "boot2")]
#[link_section = ".boot2"]
#[no_mangle]
#[used]
pub static BOOT2_FIRMWARE: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25X10CL;

pub use hal::pac;

hal::bsp_pins!(
    /// D0 supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI0 RX`    | [crate::D0Spi0Rx]           |
    /// | `UART0 TX`   | [crate::D0Uart0Tx]          |
    /// | `I2C0 SDA`   | [crate::D0I2C0Sda]          |
    /// | `PWM0 A`     | [crate::D0Pwm0A]            |
    /// | `PIO0`       | [crate::D0Pio0]             |
    /// | `PIO1`       | [crate::D0Pio1]             |
    Gpio0 {
        name: d0,
        aliases: {
            /// UART Function alias for pin [crate::Pins::d0].
            FunctionUart, PullNone: D0Uart0Tx,
            /// SPI Function alias for pin [crate::Pins::d0].
            FunctionSpi, PullNone: D0Spi0Rx,
            /// I2C Function alias for pin [crate::Pins::d0].
            FunctionI2C, PullUp: D0I2C0Sda,
            /// PWM Function alias for pin [crate::Pins::d0].
            FunctionPwm, PullNone: D0Pwm0A,
            /// PIO0 Function alias for pin [crate::Pins::d0].
            FunctionPio0, PullNone: D0Pio0,
            /// PIO1 Function alias for pin [crate::Pins::d0].
            FunctionPio1, PullNone: D0Pio1
        }
    },

    /// D1 supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI0 CSn`   | [crate::D1Spi0Csn]          |
    /// | `UART0 RX`   | [crate::D1Uart0Rx]          |
    /// | `I2C0 SCL`   | [crate::D1I2C0Scl]          |
    /// | `PWM0 B`     | [crate::D1Pwm0B]            |
    /// | `PIO0`       | [crate::D1Pio0]             |
    /// | `PIO1`       | [crate::D1Pio1]             |
    Gpio1 {
        name: d1,
        aliases: {
            /// UART Function alias for pin [crate::Pins::d1].
            FunctionUart, PullNone: D1Uart0Rx,
            /// SPI Function alias for pin [crate::Pins::d1].
            FunctionSpi, PullNone: D1Spi0Csn,
            /// I2C Function alias for pin [crate::Pins::d1].
            FunctionI2C, PullUp: D1I2C0Scl,
            /// PWM Function alias for pin [crate::Pins::d1].
            FunctionPwm, PullNone: D1Pwm0B,
            /// PIO0 Function alias for pin [crate::Pins::d1].
            FunctionPio0, PullNone: D1Pio0,
            /// PIO1 Function alias for pin [crate::Pins::d1].
            FunctionPio1, PullNone: D1Pio1
        }
    },

    /// D2 supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI0 SCK`   | [crate::D2Spi0Sck]          |
    /// | `UART0 CTS`  | [crate::D2Uart0Cts]         |
    /// | `I2C1 SDA`   | [crate::D2I2C1Sda]          |
    /// | `PWM1 A`     | [crate::D2Pwm1A]            |
    /// | `PIO0`       | [crate::D2Pio0]             |
    /// | `PIO1`       | [crate::D2Pio1]             |
    Gpio2 {
        name: d2,
        aliases: {
            /// UART Function alias for pin [crate::Pins::d2].
            FunctionUart, PullNone: D2Uart0Cts,
            /// SPI Function alias for pin [crate::Pins::d2].
            FunctionSpi, PullNone: D2Spi0Sck,
            /// I2C Function alias for pin [crate::Pins::d2].
            FunctionI2C, PullUp: D2I2C1Sda,
            /// PWM Function alias for pin [crate::Pins::d2].
            FunctionPwm, PullNone: D2Pwm1A,
            /// PIO0 Function alias for pin [crate::Pins::d2].
            FunctionPio0, PullNone: D2Pio0,
            /// PIO1 Function alias for pin [crate::Pins::d2].
            FunctionPio1, PullNone: D2Pio1
        }
    },

    /// D3 supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI0 TX`    | [crate::D3Spi0Tx]           |
    /// | `UART0 RTS`  | [crate::D3Uart0Rts]         |
    /// | `I2C1 SCL`   | [crate::D3I2C1Scl]          |
    /// | `PWM1 B`     | [crate::D3Pwm1B]            |
    /// | `PIO0`       | [crate::D3Pio0]             |
    /// | `PIO1`       | [crate::D3Pio1]             |
    Gpio3 {
        name: d3,
        aliases: {
            /// UART Function alias for pin [crate::Pins::d3].
            FunctionUart, PullNone: D3Uart0Rts,
            /// SPI Function alias for pin [crate::Pins::d3].
            FunctionSpi, PullNone: D3Spi0Tx,
            /// I2C Function alias for pin [crate::Pins::d3].
            FunctionI2C, PullUp: D3I2C1Scl,
            /// PWM Function alias for pin [crate::Pins::d3].
            FunctionPwm, PullNone: D3Pwm1B,
            /// PIO0 Function alias for pin [crate::Pins::d3].
            FunctionPio0, PullNone: D3Pio0,
            /// PIO1 Function alias for pin [crate::Pins::d3].
            FunctionPio1, PullNone: D3Pio1
        }
    },

    /// D4 supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI0 RX`    | [crate::D4Spi0Rx]           |
    /// | `UART1 TX`   | [crate::D4Uart1Tx]          |
    /// | `I2C0 SDA`   | [crate::D4I2C0Sda]          |
    /// | `PWM2 A`     | [crate::D4Pwm2A]            |
    /// | `PIO0`       | [crate::D4Pio0]             |
    /// | `PIO1`       | [crate::D4Pio1]             |
    Gpio4 {
        name: d4,
        aliases: {
            /// UART Function alias for pin [crate::Pins::d4].
            FunctionUart, PullNone: D4Uart1Tx,
            /// SPI Function alias for pin [crate::Pins::d4].
            FunctionSpi, PullNone: D4Spi0Rx,
            /// I2C Function alias for pin [crate::Pins::d4].
            FunctionI2C, PullUp: D4I2C0Sda,
            /// PWM Function alias for pin [crate::Pins::d4].
            FunctionPwm, PullNone: D4Pwm2A,
            /// PIO0 Function alias for pin [crate::Pins::d4].
            FunctionPio0, PullNone: D4Pio0,
            /// PIO1 Function alias for pin [crate::Pins::d4].
            FunctionPio1, PullNone: D4Pio1
        }
    },

    /// D5 supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI0 CSn`   | [crate::D5Spi0Csn]          |
    /// | `UART1 RX`   | [crate::D5Uart1Rx]          |
    /// | `I2C0 SCL`   | [crate::D5I2C0Scl]          |
    /// | `PWM2 B`     | [crate::D5Pwm2B]            |
    /// | `PIO0`       | [crate::D5Pio0]             |
    /// | `PIO1`       | [crate::D5Pio1]             |
    Gpio5 {
        name: d5,
        aliases: {
            /// UART Function alias for pin [crate::Pins::d5].
            FunctionUart, PullNone: D5Uart1Rx,
            /// SPI Function alias for pin [crate::Pins::d5].
            FunctionSpi, PullNone: D5Spi0Csn,
            /// I2C Function alias for pin [crate::Pins::d5].
            FunctionI2C, PullUp: D5I2C0Scl,
            /// PWM Function alias for pin [crate::Pins::d5].
            FunctionPwm, PullNone: D5Pwm2B,
            /// PIO0 Function alias for pin [crate::Pins::d5].
            FunctionPio0, PullNone: D5Pio0,
            /// PIO1 Function alias for pin [crate::Pins::d5].
            FunctionPio1, PullNone: D5Pio1
        }
    },

    /// D6 supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI0 SCK`   | [crate::D6Spi0Sck]          |
    /// | `UART1 CTS`  | [crate::D6Uart1Cts]         |
    /// | `I2C1 SDA`   | [crate::D6I2C1Sda]          |
    /// | `PWM3 A`     | [crate::D6Pwm3A]            |
    /// | `PIO0`       | [crate::D6Pio0]             |
    /// | `PIO1`       | [crate::D6Pio1]             |
    Gpio6 {
        name: d6,
        aliases: {
            /// UART Function alias for pin [crate::Pins::d6].
            FunctionUart, PullNone: D6Uart1Cts,
            /// SPI Function alias for pin [crate::Pins::d6].
            FunctionSpi, PullNone: D6Spi0Sck,
            /// I2C Function alias for pin [crate::Pins::d6].
            FunctionI2C, PullUp: D6I2C1Sda,
            /// PWM Function alias for pin [crate::Pins::d6].
            FunctionPwm, PullNone: D6Pwm3A,
            /// PIO0 Function alias for pin [crate::Pins::d6].
            FunctionPio0, PullNone: D6Pio0,
            /// PIO1 Function alias for pin [crate::Pins::d6].
            FunctionPio1, PullNone: D6Pio1
        }
    },

    /// D7 supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI0 TX`    | [crate::D7Spi0Tx]           |
    /// | `UART1 RTS`  | [crate::D7Uart1Rts]         |
    /// | `I2C1 SCL`   | [crate::D7I2C1Scl]          |
    /// | `PWM3 B`     | [crate::D7Pwm3B]            |
    /// | `PIO0`       | [crate::D7Pio0]             |
    /// | `PIO1`       | [crate::D7Pio1]             |
    Gpio7 {
        name: d7,
        aliases: {
            /// UART Function alias for pin [crate::Pins::d7].
            FunctionUart, PullNone: D7Uart1Rts,
            /// SPI Function alias for pin [crate::Pins::d7].
            FunctionSpi, PullNone: D7Spi0Tx,
            /// I2C Function alias for pin [crate::Pins::d7].
            FunctionI2C, PullUp: D7I2C1Scl,
            /// PWM Function alias for pin [crate::Pins::d7].
            FunctionPwm, PullNone: D7Pwm3B,
            /// PIO0 Function alias for pin [crate::Pins::d7].
            FunctionPio0, PullNone: D7Pio0,
            /// PIO1 Function alias for pin [crate::Pins::d7].
            FunctionPio1, PullNone: D7Pio1
        }
    },

    /// D8 supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI1 RX`    | [crate::D8Spi1Rx]           |
    /// | `UART1 TX`   | [crate::D8Uart1Tx]          |
    /// | `I2C0 SDA`   | [crate::D8I2C0Sda]          |
    /// | `PWM4 A`     | [crate::D8Pwm4A]            |
    /// | `PIO0`       | [crate::D8Pio0]             |
    /// | `PIO1`       | [crate::D8Pio1]             |
    Gpio8 {
        name: d8,
        aliases: {
            /// UART Function alias for pin [crate::Pins::d8].
            FunctionUart, PullNone: D8Uart1Tx,
            /// SPI Function alias for pin [crate::Pins::d8].
            FunctionSpi, PullNone: D8Spi1Rx,
            /// I2C Function alias for pin [crate::Pins::d8].
            FunctionI2C, PullUp: D8I2C0Sda,
            /// PWM Function alias for pin [crate::Pins::d8].
            FunctionPwm, PullNone: D8Pwm4A,
            /// PIO0 Function alias for pin [crate::Pins::d8].
            FunctionPio0, PullNone: D8Pio0,
            /// PIO1 Function alias for pin [crate::Pins::d8].
            FunctionPio1, PullNone: D8Pio1
        }
    },

    /// D9 supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI1 CSn`   | [crate::D9Spi1Csn]          |
    /// | `UART1 RX`   | [crate::D9Uart1Rx]          |
    /// | `I2C0 SCL`   | [crate::D9I2C0Scl]          |
    /// | `PWM4 B`     | [crate::D9Pwm4B]            |
    /// | `PIO0`       | [crate::D9Pio0]             |
    /// | `PIO1`       | [crate::D9Pio1]             |
    Gpio9 {
        name: d9,
        aliases: {
            /// UART Function alias for pin [crate::Pins::d9].
            FunctionUart, PullNone: D9Uart1Rx,
            /// SPI Function alias for pin [crate::Pins::d9].
            FunctionSpi, PullNone: D9Spi1Csn,
            /// I2C Function alias for pin [crate::Pins::d9].
            FunctionI2C, PullUp: D9I2C0Scl,
            /// PWM Function alias for pin [crate::Pins::d9].
            FunctionPwm, PullNone: D9Pwm4B,
            /// PIO0 Function alias for pin [crate::Pins::d9].
            FunctionPio0, PullNone: D9Pio0,
            /// PIO1 Function alias for pin [crate::Pins::d9].
            FunctionPio1, PullNone: D9Pio1
        }
    },

    /// D11 supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI1 TX`    | [crate::D11Spi1Tx]          |
    /// | `UART1 RTS`  | [crate::D11Uart1Rts]        |
    /// | `I2C1 SCL`   | [crate::D11I2C1Scl]         |
    /// | `PWM5 B`     | [crate::D11Pwm5B]           |
    /// | `PIO0`       | [crate::D11Pio0]            |
    /// | `PIO1`       | [crate::D11Pio1]            |
    Gpio11 {
        name: d11,
        aliases: {
            /// UART Function alias for pin [crate::Pins::d11].
            FunctionUart, PullNone: D11Uart1Rts,
            /// SPI Function alias for pin [crate::Pins::d11].
            FunctionSpi, PullNone: D11Spi1Tx,
            /// I2C Function alias for pin [crate::Pins::d11].
            FunctionI2C, PullUp: D11I2C1Scl,
            /// PWM Function alias for pin [crate::Pins::d11].
            FunctionPwm, PullNone: D11Pwm5B,
            /// PIO0 Function alias for pin [crate::Pins::d11].
            FunctionPio0, PullNone: D11Pio0,
            /// PIO1 Function alias for pin [crate::Pins::d11].
            FunctionPio1, PullNone: D11Pio1
        }
    },

    /// D12 supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI1 RX`    | [crate::D12Spi1Rx]          |
    /// | `UART0 TX`   | [crate::D12Uart0Tx]         |
    /// | `I2C0 SDA`   | [crate::D12I2C0Sda]         |
    /// | `PWM6 A`     | [crate::D12Pwm6A]           |
    /// | `PIO0`       | [crate::D12Pio0]            |
    /// | `PIO1`       | [crate::D12Pio1]            |
    Gpio12 {
        name: d12,
        aliases: {
            /// UART Function alias for pin [crate::Pins::d12].
            FunctionUart, PullNone: D12Uart0Tx,
            /// SPI Function alias for pin [crate::Pins::d12].
            FunctionSpi, PullNone: D12Spi1Rx,
            /// I2C Function alias for pin [crate::Pins::d12].
            FunctionI2C, PullUp: D12I2C0Sda,
            /// PWM Function alias for pin [crate::Pins::d12].
            FunctionPwm, PullNone: D12Pwm6A,
            /// PIO0 Function alias for pin [crate::Pins::d12].
            FunctionPio0, PullNone: D12Pio0,
            /// PIO1 Function alias for pin [crate::Pins::d12].
            FunctionPio1, PullNone: D12Pio1
        }
    },

    /// D13 supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI1 CSn`   | [crate::D13Spi1Csn]         |
    /// | `UART0 RX`   | [crate::D13Uart0Rx]         |
    /// | `I2C0 SCL`   | [crate::D13I2C0Scl]         |
    /// | `PWM6 B`     | [crate::D13Pwm6B]           |
    /// | `PIO0`       | [crate::D13Pio0]            |
    /// | `PIO1`       | [crate::D13Pio1]            |
    Gpio13 {
        name: d13,
        aliases: {
            /// UART Function alias for pin [crate::Pins::d13].
            FunctionUart, PullNone: D13Uart0Rx,
            /// SPI Function alias for pin [crate::Pins::d13].
            FunctionSpi, PullNone: D13Spi1Csn,
            /// I2C Function alias for pin [crate::Pins::d13].
            FunctionI2C, PullUp: D13I2C0Scl,
            /// PWM Function alias for pin [crate::Pins::d13].
            FunctionPwm, PullNone: D13Pwm6B,
            /// PIO0 Function alias for pin [crate::Pins::d13].
            FunctionPio0, PullNone: D13Pio0,
            /// PIO1 Function alias for pin [crate::Pins::d13].
            FunctionPio1, PullNone: D13Pio1
        }
    },

    /// D14 supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI1 SCK`   | [crate::D14Spi1Sck]         |
    /// | `UART0 CTS`  | [crate::D14Uart0Cts]        |
    /// | `I2C1 SDA`   | [crate::D14I2C1Sda]         |
    /// | `PWM7 A`     | [crate::D14Pwm7A]           |
    /// | `PIO0`       | [crate::D14Pio0]            |
    /// | `PIO1`       | [crate::D14Pio1]            |
    Gpio14 {
        name: d14,
        aliases: {
            /// UART Function alias for pin [crate::Pins::d14].
            FunctionUart, PullNone: D14Uart0Cts,
            /// SPI Function alias for pin [crate::Pins::d14].
            FunctionSpi, PullNone: D14Spi1Sck,
            /// I2C Function alias for pin [crate::Pins::d14].
            FunctionI2C, PullUp: D14I2C1Sda,
            /// PWM Function alias for pin [crate::Pins::d14].
            FunctionPwm, PullNone: D14Pwm7A,
            /// PIO0 Function alias for pin [crate::Pins::d14].
            FunctionPio0, PullNone: D14Pio0,
            /// PIO1 Function alias for pin [crate::Pins::d14].
            FunctionPio1, PullNone: D14Pio1
        }
    },

    /// GPIO 16 is connected to the red LED, active low,
    /// and supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `PWM0 A`     | [crate::LedRedPwm0A]        |
    Gpio16 {
        name: led_red,
        aliases: {
            /// PWM Function alias for pin [crate::Pins::led_red].
            FunctionPwm, PullNone: LedRedPwm0A
        }
     },

    /// GPIO 17 is connected to the green LED, active low,
    /// and supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `PWM0 B`     | [crate::LedGrnPwm0B]        |
    Gpio17 {
        name: led_green,
        aliases: {
            /// PWM Function alias for pin [crate::Pins::led_green].
            FunctionPwm, PullNone: LedGrnPwm0B
        }
     },

    /// GPIO 18 is connected to the blue LED, active low,
    /// and supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `PWM1 A`     | [crate::LedBluPwm1A]        |
    ///
    Gpio18 {
        name: led_blue,
        aliases: {
            /// PWM Function alias for pin [crate::Pins::led_green].
            FunctionPwm, PullNone: LedBluPwm1A
        }
     },

    /// D10 supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI0 CSn`   | [crate::D10Spi0Csn]         |
    /// | `UART1 RX`   | [crate::D10Uart1Rx]         |
    /// | `I2C0 SCL`   | [crate::D10I2C0Scl]         |
    /// | `PWM2 B`     | [crate::D10Pwm2B]           |
    /// | `PIO0`       | [crate::D10Pio0]            |
    /// | `PIO1`       | [crate::D10Pio1]            |
    Gpio21 {
        name: d10,
        aliases: {
            /// UART Function alias for pin [crate::Pins::d10].
            FunctionUart, PullNone: D10Uart1Rx,
            /// SPI Function alias for pin [crate::Pins::d10].
            FunctionSpi, PullNone: D10Spi0Csn,
            /// I2C Function alias for pin [crate::Pins::d10].
            FunctionI2C, PullUp: D10I2C0Scl,
            /// PWM Function alias for pin [crate::Pins::d10].
            FunctionPwm, PullNone: D10Pwm2B,
            /// PIO0 Function alias for pin [crate::Pins::d10].
            FunctionPio0, PullNone: D10Pio0,
            /// PIO1 Function alias for pin [crate::Pins::d10].
            FunctionPio1, PullNone: D10Pio1
        }
    },

    /// MOSI supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI0 TX`    | [crate::Mosi]               |
    /// | `UART1 RTS`  | [crate::MosiUart1Rts]       |
    /// | `I2C1 SCL`   | [crate::MosiI2C1Scl]        |
    /// | `PWM3 B`     | [crate::MosiPwm3B]          |
    /// | `PIO0`       | [crate::MosiPio0]           |
    /// | `PIO1`       | [crate::MosiPio1]           |
    Gpio23 {
        name: mosi,
        aliases: {
            /// UART Function alias for pin [crate::Pins::mosi].
            FunctionUart, PullNone: MosiUart1Rts,
            /// SPI Function alias for pin [crate::Pins::mosi].
            FunctionSpi, PullNone: Mosi,
            /// I2C Function alias for pin [crate::Pins::mosi].
            FunctionI2C, PullUp: MosiI2C1Scl,
            /// PWM Function alias for pin [crate::Pins::mosi].
            FunctionPwm, PullNone: MosiPwm3B,
            /// PIO0 Function alias for pin [crate::Pins::mosi].
            FunctionPio0, PullNone: MosiPio0,
            /// PIO1 Function alias for pin [crate::Pins::mosi].
            FunctionPio1, PullNone: MosiPio1
        }
    },

    /// MISO supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI0 RX`    | [crate::Miso]               |
    /// | `UART1 TX`   | [crate::MisoUart1Tx]        |
    /// | `I2C0 SDA`   | [crate::MisoI2C0Sda]        |
    /// | `PWM2 A`     | [crate::MisoPwm2A]          |
    /// | `PIO0`       | [crate::MisoPio0]           |
    /// | `PIO1`       | [crate::MisoPio1]           |
    Gpio20 {
        name: miso,
        aliases: {
            /// UART Function alias for pin [crate::Pins::miso].
            FunctionUart, PullNone: MisoUart1Tx,
            /// SPI Function alias for pin [crate::Pins::miso].
            FunctionSpi, PullNone: Miso,
            /// I2C Function alias for pin [crate::Pins::miso].
            FunctionI2C, PullUp: MisoI2C0Sda,
            /// PWM Function alias for pin [crate::Pins::miso].
            FunctionPwm, PullNone: MisoPwm2A,
            /// PIO0 Function alias for pin [crate::Pins::miso].
            FunctionPio0, PullNone: MisoPio0,
            /// PIO1 Function alias for pin [crate::Pins::miso].
            FunctionPio1, PullNone: MisoPio1
        }
    },

    /// SCK supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI0 SCK`   | [crate::Sck]                |
    /// | `UART1 CTS`  | [crate::SckUart1Cts]        |
    /// | `I2C1 SDA`   | [crate::SckI2C1Sda]         |
    /// | `PWM3 A`     | [crate::SckPwm3A]           |
    /// | `PIO0`       | [crate::SckPio0]            |
    /// | `PIO1`       | [crate::SckPio1]            |
    Gpio22 {
        name: sck,
        aliases: {
            /// UART Function alias for pin [crate::Pins::sck].
            FunctionUart, PullNone: SckUart1Cts,
            /// SPI Function alias for pin [crate::Pins::sck].
            FunctionSpi, PullNone: Sck,
            /// I2C Function alias for pin [crate::Pins::sck].
            FunctionI2C, PullUp: SckI2C1Sda,
            /// PWM Function alias for pin [crate::Pins::sck].
            FunctionPwm, PullNone: SckPwm3A,
            /// PIO0 Function alias for pin [crate::Pins::sck].
            FunctionPio0, PullNone: SckPio0,
            /// PIO1 Function alias for pin [crate::Pins::sck].
            FunctionPio1, PullNone: SckPio1
        }
    },

    /// A0 supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI1 SCK`   | [crate::A0Spi1Sck]          |
    /// | `UART1 CTS`  | [crate::A0Uart1Cts]         |
    /// | `I2C1 SDA`   | [crate::A0I2C1Sda]          |
    /// | `PWM5 A`     | [crate::A0Pwm5A]            |
    /// | `PIO0`       | [crate::A0Pio0]             |
    /// | `PIO1`       | [crate::A0Pio1]             |
    Gpio26  {
        name: a0,
        aliases: {
            /// UART Function alias for pin [crate::Pins::a0].
            FunctionUart, PullNone: A0Uart1Cts,
            /// SPI Function alias for pin [crate::Pins::a0].
            FunctionSpi, PullNone: A0Spi1Sck,
            /// I2C Function alias for pin [crate::Pins::a0].
            FunctionI2C, PullUp: A0I2C1Sda,
            /// PWM Function alias for pin [crate::Pins::a0].
            FunctionPwm, PullNone: A0Pwm5A,
            /// PIO0 Function alias for pin [crate::Pins::a0].
            FunctionPio0, PullNone: A0Pio0,
            /// PIO1 Function alias for pin [crate::Pins::a0].
            FunctionPio1, PullNone: A0Pio1
        }
    },

    /// A1 supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI1 TX`    | [crate::A1Spi1Tx]           |
    /// | `UART1 RTS`  | [crate::A1Uart1Rts]         |
    /// | `I2C1 SCL`   | [crate::A1I2C1Scl]          |
    /// | `PWM5 B`     | [crate::A1Pwm5B]            |
    /// | `PIO0`       | [crate::A1Pio0]             |
    /// | `PIO1`       | [crate::A1Pio1]             |
    Gpio27  {
        name: a1,
        aliases: {
            /// UART Function alias for pin [crate::Pins::a1].
            FunctionUart, PullNone: A1Uart1Rts,
            /// SPI Function alias for pin [crate::Pins::a1].
            FunctionSpi, PullNone: A1Spi1Tx,
            /// I2C Function alias for pin [crate::Pins::a1].
            FunctionI2C, PullUp: A1I2C1Scl,
            /// PWM Function alias for pin [crate::Pins::a1].
            FunctionPwm, PullNone: A1Pwm5B,
            /// PIO0 Function alias for pin [crate::Pins::a1].
            FunctionPio0, PullNone: A1Pio0,
            /// PIO1 Function alias for pin [crate::Pins::a1].
            FunctionPio1, PullNone: A1Pio1
        }
    },

    /// A2 supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI1 RX`    | [crate::A2Spi1Rx]           |
    /// | `UART0 TX`   | [crate::A2Uart0Tx]          |
    /// | `I2C0 SDA`   | [crate::A2I2C0Sda]          |
    /// | `PWM6 A`     | [crate::A2Pwm6A]            |
    /// | `PIO0`       | [crate::A2Pio0]             |
    /// | `PIO1`       | [crate::A2Pio1]             |
    Gpio28  {
        name: a2,
        aliases: {
            /// UART Function alias for pin [crate::Pins::a2].
            FunctionUart, PullNone: A2Uart0Tx,
            /// SPI Function alias for pin [crate::Pins::a2].
            FunctionSpi, PullNone: A2Spi1Rx,
            /// I2C Function alias for pin [crate::Pins::a2].
            FunctionI2C, PullUp: A2I2C0Sda,
            /// PWM Function alias for pin [crate::Pins::a2].
            FunctionPwm, PullNone: A2Pwm6A,
            /// PIO0 Function alias for pin [crate::Pins::a2].
            FunctionPio0, PullNone: A2Pio0,
            /// PIO1 Function alias for pin [crate::Pins::a2].
            FunctionPio1, PullNone: A2Pio1
        }
    },

    /// A3 supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI1 CS`    | [crate::A3Spi1CSn]          |
    /// | `UART0 RX`   | [crate::A3Uart0Rx]          |
    /// | `I2C0 SCL`   | [crate::A3I2C0Scl]          |
    /// | `PWM6 B`     | [crate::A3Pwm6B]            |
    /// | `PIO0`       | [crate::A3Pio0]             |
    /// | `PIO1`       | [crate::A3Pio1]             |
    Gpio29  {
        name: a3,
        aliases: {
            /// UART Function alias for pin [crate::Pins::a3].
            FunctionUart, PullNone: A3Uart0Rx,
            /// SPI Function alias for pin [crate::Pins::a3].
            FunctionSpi, PullNone: A3Spi1CSn,
            /// I2C Function alias for pin [crate::Pins::a3].
            FunctionI2C, PullUp: A3I2C0Scl,
            /// PWM Function alias for pin [crate::Pins::a3].
            FunctionPwm, PullNone: A3Pwm6B,
            /// PIO0 Function alias for pin [crate::Pins::a3].
            FunctionPio0, PullNone: A3Pio0,
            /// PIO1 Function alias for pin [crate::Pins::a3].
            FunctionPio1, PullNone: A3Pio1
        }
    },
);

pub const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;
