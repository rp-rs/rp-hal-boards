#![no_std]

pub use rp2040_hal as hal;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use hal::entry;

/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
#[cfg(feature = "boot2")]
#[link_section = ".boot2"]
#[no_mangle]
#[used]
pub static BOOT2_FIRMWARE: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

pub use hal::pac;

hal::bsp_pins!(
    Gpio0 { name: motor_3_enc_a },
    Gpio1 { name: motor_3_enc_b },
    Gpio2 { name: motor_3_phase },
    Gpio3 { name: motor_3_enable },
    Gpio4 { name: left_motor_enc_a },
    Gpio5 { name: left_motor_enc_b },
    Gpio6 { name: left_motor_phase },
    Gpio7 { name: left_motor_enable },
    Gpio8 { name: motor_4_enc_a },
    Gpio9 { name: motor_4_enc_b },
    Gpio10 { name: motor_4_phase },
    Gpio11 { name: motor_4_enable },
    Gpio12 { name: right_motor_enc_a },
    Gpio13 { name: right_motor_enc_b },
    Gpio14 { name: right_motor_phase },
    Gpio15 { name: right_motor_enable },
    Gpio16 { name: servo_1 },
    Gpio17 { name: servo_2 },
    Gpio18 { name: qwiic_sda },
    Gpio19 { name: qwiic_scl },
    Gpio20 { name: range_trigger },
    Gpio21 { name: range_echo },
    Gpio22 { name: user_button },
    Gpio23 { name: wireless_power },
    Gpio24 { name: wireless_data },
    Gpio25 { name: wireless_cs }
    Gpio26 { name: line_follower_left },
    Gpio27 { name: line_follower_right },
    Gpio28 { name: vin_measure },
    Gpio29 { name: wireless_clk }
);

pub const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;
