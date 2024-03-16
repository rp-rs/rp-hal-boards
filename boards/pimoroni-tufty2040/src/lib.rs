#![no_std]

pub extern crate rp2040_hal as hal;

pub use hal::pac;

use display_interface::{DataFormat, DisplayError, WriteOnlyDataCommand};
use embedded_hal::digital::v2::OutputPin;
use fugit::HertzU32;
use hal::dma::{single_buffer, Channel, ChannelIndex, WriteTarget};
use hal::gpio::PinId;
use hal::pio::{
    Buffers, PIOBuilder, PIOExt, PinDir, PinState, StateMachineIndex, Tx, UninitStateMachine,
};
use pio_proc::pio_file;

#[cfg(feature = "rt")]
pub use rp2040_hal::entry;

/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
#[cfg(feature = "boot2")]
#[link_section = ".boot2"]
#[no_mangle]
#[used]
pub static BOOT2_FIRMWARE: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

hal::bsp_pins!(
    Gpio0 {
        name: gpio0,
        aliases: {
            /// UART Function alias for pin [Pins::gpio0].
            FunctionUart, PullNone: UartTx
        }
    },
    Gpio1 {
        name: gpio1,
        aliases: {
            /// UART Function alias for pin [Pins::gpio1].
            FunctionUart, PullNone: UartRx
        }
    },
    Gpio2 { name: lcd_backlight },
    Gpio3 { name: i2c_int },
    Gpio4 {
        name: gpio4,
        aliases: {
            /// I2C Function alias for pin [Pins::gpio4].
            FunctionI2C, PullUp: I2cSda
        }
    },
    Gpio5 {
        name: gpio5,
        aliases: {
            /// I2C Function alias for pin [Pins::gpio5].
            FunctionI2C, PullUp: I2cScl
        }
    },
    Gpio6 { name: sw_down },
    Gpio7 { name: sw_a },
    Gpio8 { name: sw_b },
    Gpio9 { name: sw_c },
    Gpio10 { name: lcd_cs },
    Gpio11 { name: lcd_dc },
    Gpio12 { name: lcd_wr },
    Gpio13 { name: lcd_rd },
    Gpio14 { name: lcd_db0 },
    Gpio15 { name: lcd_db1 },
    Gpio16 { name: lcd_db2 },
    Gpio17 { name: lcd_db3 },
    Gpio18 { name: lcd_db4 },
    Gpio19 { name: lcd_db5 },
    Gpio20 { name: lcd_db6 },
    Gpio21 { name: lcd_db7 },
    Gpio22 { name: sw_up },
    Gpio23 { name: user_sw },
    Gpio24 { name: vbus_detect },
    Gpio25 { name: led },
    Gpio26 { name: light_sense },
    Gpio27 { name: sensor_power },
    Gpio28 { name: vref_1v24 },
    Gpio29 { name: vbat_sense },
);

pub const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;

#[inline]
fn set_pin_bit<P: OutputPin>(pin: &mut P, bit: u8, value: u8) -> Result<(), DisplayError> {
    pin.set_state(((bit & value) != 0).into())
        .map_err(|_| DisplayError::BusWriteError)
}

struct WriteBytes<T>(T);

// Allow DMA to do byte-size writes to an existing target,
// SAFETY: This is only used with the PIO as a target, which is valid to write
// byte-width.
unsafe impl<T: WriteTarget> WriteTarget for WriteBytes<T> {
    type TransmittedWord = u8;

    #[inline]
    fn tx_treq() -> Option<u8> {
        T::tx_treq()
    }

    #[inline]
    fn tx_address_count(&mut self) -> (u32, u32) {
        self.0.tx_address_count()
    }

    #[inline]
    fn tx_increment(&self) -> bool {
        self.0.tx_increment()
    }
}

pub trait DisplayDataLines {
    fn flush(&mut self) {}

    fn write_u8(&mut self, value: u8) -> Result<(), DisplayError>;

    fn write_slice(&mut self, data: &[u8]) -> Result<(), DisplayError> {
        for b in data.iter().copied() {
            self.write_u8(b)?;
        }

        Ok(())
    }

    fn write_format(&mut self, data: DataFormat<'_>) -> Result<(), DisplayError> {
        match data {
            DataFormat::U8(bytes) => self.write_slice(bytes)?,
            DataFormat::U16(items) => {
                for value in items.iter().copied() {
                    self.write_u8(value as u8)?;
                    self.write_u8((value >> 8) as u8)?;
                }
            }
            DataFormat::U16BE(items) => {
                for value in items.iter().copied() {
                    self.write_u8((value >> 8) as u8)?;
                    self.write_u8(value as u8)?;
                }
            }
            DataFormat::U16LE(items) => {
                for value in items.iter().copied() {
                    self.write_u8(value as u8)?;
                    self.write_u8((value >> 8) as u8)?;
                }
            }
            DataFormat::U8Iter(iter) => {
                for value in iter {
                    self.write_u8(value)?;
                }
            }
            DataFormat::U16BEIter(iter) => {
                for value in iter {
                    self.write_u8((value >> 8) as u8)?;
                    self.write_u8(value as u8)?;
                }
            }
            DataFormat::U16LEIter(iter) => {
                for value in iter {
                    self.write_u8(value as u8)?;
                    self.write_u8((value >> 8) as u8)?;
                }
            }
            _ => unimplemented!(),
        }

        Ok(())
    }
}

pub struct GpioDataLines<WR, D0, D1, D2, D3, D4, D5, D6, D7> {
    pub wr: WR,
    pub d0: D0,
    pub d1: D1,
    pub d2: D2,
    pub d3: D3,
    pub d4: D4,
    pub d5: D5,
    pub d6: D6,
    pub d7: D7,
}

impl<
        WR: OutputPin,
        D0: OutputPin,
        D1: OutputPin,
        D2: OutputPin,
        D3: OutputPin,
        D4: OutputPin,
        D5: OutputPin,
        D6: OutputPin,
        D7: OutputPin,
    > GpioDataLines<WR, D0, D1, D2, D3, D4, D5, D6, D7>
{
    #[inline]
    fn write_u8_inner(&mut self, value: u8) -> Result<(), DisplayError> {
        set_pin_bit(&mut self.d0, value, 1 << 0)?;
        set_pin_bit(&mut self.d1, value, 1 << 1)?;
        set_pin_bit(&mut self.d2, value, 1 << 2)?;
        set_pin_bit(&mut self.d3, value, 1 << 3)?;
        set_pin_bit(&mut self.d4, value, 1 << 4)?;
        set_pin_bit(&mut self.d5, value, 1 << 5)?;
        set_pin_bit(&mut self.d6, value, 1 << 6)?;
        set_pin_bit(&mut self.d7, value, 1 << 7)?;
        Ok(())
    }
}

impl<
        WR: OutputPin,
        D0: OutputPin,
        D1: OutputPin,
        D2: OutputPin,
        D3: OutputPin,
        D4: OutputPin,
        D5: OutputPin,
        D6: OutputPin,
        D7: OutputPin,
    > DisplayDataLines for GpioDataLines<WR, D0, D1, D2, D3, D4, D5, D6, D7>
{
    fn write_u8(&mut self, value: u8) -> Result<(), DisplayError> {
        self.wr.set_low().map_err(|_| DisplayError::BusWriteError)?;
        let err = self.write_u8_inner(value);
        self.wr.set_high().ok();
        err
    }
}

type PioTx<P, SM, CH> = (Tx<(P, SM)>, Channel<CH>);

pub struct PioDataLines<P: PIOExt, SM: StateMachineIndex, CH: ChannelIndex> {
    tx: Option<PioTx<P, SM, CH>>,
}

impl<P: PIOExt, SM: StateMachineIndex, CH: ChannelIndex> PioDataLines<P, SM, CH> {
    pub fn new(
        pio: &mut hal::pio::PIO<P>,
        sys_freq: HertzU32,
        wr: impl PinId,
        d0: impl PinId,
        sm: UninitStateMachine<(P, SM)>,
        ch: Channel<CH>,
    ) -> PioDataLines<P, SM, CH> {
        let d0 = d0.as_dyn().num;
        let wr = wr.as_dyn().num;

        let max_pio_clk = HertzU32::MHz(32);
        let divider = (sys_freq + max_pio_clk - HertzU32::Hz(1)) / max_pio_clk;

        let program = pio_file!("./src/st7789_parallel.pio");
        let program = pio.install(&program.program).unwrap();
        let (mut sm, _rx, tx) = PIOBuilder::from_program(program)
            .out_pins(d0, 8)
            .side_set_pin_base(wr)
            .buffers(Buffers::OnlyTx)
            .pull_threshold(8)
            .autopull(true)
            .clock_divisor_fixed_point(divider as u16, 0)
            .build(sm);
        sm.set_pindirs([
            (d0, PinDir::Output),
            (d0 + 1, PinDir::Output),
            (d0 + 2, PinDir::Output),
            (d0 + 3, PinDir::Output),
            (d0 + 4, PinDir::Output),
            (d0 + 5, PinDir::Output),
            (d0 + 6, PinDir::Output),
            (d0 + 7, PinDir::Output),
            (wr, PinDir::Output),
        ]);
        sm.set_pins([(wr, PinState::High)]);
        sm.start();

        PioDataLines { tx: Some((tx, ch)) }
    }
}

impl<P: PIOExt, SM: StateMachineIndex, CH: ChannelIndex> DisplayDataLines
    for PioDataLines<P, SM, CH>
{
    fn flush(&mut self) {
        if let Some((tx, _)) = self.tx.as_mut() {
            while !tx.is_empty() {}
        }
    }

    fn write_u8(&mut self, value: u8) -> Result<(), DisplayError> {
        if let Some((tx, _)) = self.tx.as_mut() {
            while !tx.write(value as u32) {}
            Ok(())
        } else {
            Err(DisplayError::BusWriteError)
        }
    }

    fn write_slice(&mut self, data: &[u8]) -> Result<(), DisplayError> {
        // SAFETY: transmute away lifetime, since we will always wait for DMA completion here.
        let data: &'static [u8] = unsafe { core::mem::transmute(data) };

        let (tx, ch) = self.tx.take().expect("DMA already in use");
        let xfer = single_buffer::Config::new(ch, data, WriteBytes(tx)).start();
        let (ch, _, WriteBytes(tx)) = xfer.wait();
        self.tx = Some((tx, ch));
        Ok(())
    }
}

pub struct ParallelDisplayInterface<CS, DC, D> {
    cs: CS,
    dc: DC,
    data_lines: D,
}

impl<CS: OutputPin, DC: OutputPin, D: DisplayDataLines> ParallelDisplayInterface<CS, DC, D> {
    pub fn new(cs: CS, dc: DC, data_lines: D) -> ParallelDisplayInterface<CS, DC, D> {
        ParallelDisplayInterface { cs, dc, data_lines }
    }
}

impl<CS: OutputPin, DC: OutputPin, D: DisplayDataLines> WriteOnlyDataCommand
    for ParallelDisplayInterface<CS, DC, D>
{
    fn send_commands(&mut self, cmds: DataFormat<'_>) -> Result<(), DisplayError> {
        self.cs.set_low().map_err(|_| DisplayError::CSError)?;
        self.dc.set_low().map_err(|_| DisplayError::DCError)?;

        let err = self.data_lines.write_format(cmds);
        self.data_lines.flush();

        err
    }

    fn send_data(&mut self, buf: DataFormat<'_>) -> Result<(), DisplayError> {
        self.dc.set_high().map_err(|_| DisplayError::DCError)?;

        let err = self.data_lines.write_format(buf);
        self.data_lines.flush();

        err
    }
}

pub struct DummyPin;

impl OutputPin for DummyPin {
    type Error = ();

    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}
