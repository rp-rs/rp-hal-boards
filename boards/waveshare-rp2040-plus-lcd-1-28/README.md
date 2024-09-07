# [waveshare-rp2040-plus-lcd-1-28] - Board Support for the [Waveshare RP2040 Plus LCD 1.28]

You should include this crate if you are writing code that you want to run on
an [Waveshare RP2040 Plus LCD 1.28] - a very small RP2040 breakout board with USB-C, 
a 65K IPS LCD 240x240, 16MBit Flash and 1A battery charger from Waveshare.

This crate includes the [rp2040-hal], but also configures each pin of the
RP2040 chip according to how it is connected up on the Feather.

[Waveshare RP2040 Plus]: https://www.waveshare.com/wiki/RP2040-Plus
[Waveshare LCD 1.28]: https://www.waveshare.com/wiki/1.28inch_LCD_Module
[waveshare-rp2040-lcd-1-28]: https://github.com/rp-rs/rp-hal-boards/tree/main/boards/waveshare-plus-rp2040-lcd-1-28
[rp2040-hal]: https://github.com/rp-rs/rp-hal/tree/main/rp2040-hal
[Raspberry Silicon RP2040]: https://www.raspberrypi.org/products/rp2040/

## Connecting the Waveshare 1.28-inch LCD Module to the Waveshare RP2040 Plus

This guide shows how to wire the **Waveshare 1.28-inch LCD Module** to the **Waveshare RP2040 Plus** and initialize the SPI interface in Rust. Follow the pin connections described below to ensure correct functionality.

### Pin Connections

| **RP2040 Pin** | **LCD Module Pin** | **Description** |
| --- | --- | --- |
| **GP16** | **DC (Data/Command)** | Data/Command control pin |
| **GP17** | **CS (Chip Select)** | SPI Chip Select |
| **GP18** | **CLK (Clock)** | SPI Clock (SCK) |
| **GP19** | **MOSI (Master Out Slave In)** | SPI Data |
| **GP20** | **RST (Reset)** | LCD Reset (Active Low) |
| **GP21** | **BL (Backlight)** | Control for the backlight |
| **3.3V** | **VCC** | Power supply for the LCD (3.3V) |
| **GND** | **GND** | Ground for the LCD |

### Detailed Pin Usage

1.  **VCC (3.3V)**: Connect the **VCC** pin on the LCD module to the **3.3V** pin on the RP2040 Plus to power the LCD.
2.  **GND (Ground)**: Connect the **GND** pin on the LCD to any **GND** pin on the RP2040 Plus.
3.  **DC (Data/Command - GP16)**: Used to select between sending data or a command to the LCD. Set to **high** for data and **low** for commands.
4.  **CS (Chip Select - GP17)**: This is the chip select for the SPI communication. Set this pin to **low** when communicating with the LCD.
5.  **CLK (Clock - GP18)**: The clock signal for SPI communication, set to function as **SPI clock (SCK)**.
6.  **MOSI (Master Out Slave In - GP19)**: The data line for SPI communication from the RP2040 to the LCD.
7.  **RST (Reset - GP20)**: Used to reset the LCD. This pin should be held **high** during normal operation and set to **low** to reset.
8.  **BL (Backlight - GP21)**: This controls the backlight of the LCD. Set **low** to turn the backlight off or **high** to turn it on.

### Code Example

Here is a basic initialization of the LCD pins and SPI communication in Rust:

rust

Copy code

`// Initialize LCD pins let lcd_dc = pins.gp16.into_push_pull_output();  // Data/Command pin let lcd_cs = pins.gp17.into_push_pull_output();  // Chip Select pin let lcd_clk = pins.gp18.into_function::<hal::gpio::FunctionSpi>();  // SPI Clock pin let lcd_mosi = pins.gp19.into_function::<hal::gpio::FunctionSpi>(); // SPI MOSI pin let lcd_rst = pins.gp20.into_push_pull_output_in_state(hal::gpio::PinState::High);  // Reset pin let mut lcd_bl = pins.gp21.into_push_pull_output_in_state(hal::gpio::PinState::Low); // Backlight control pin  // Initialize SPI let spi = hal::Spi::<_, _, _, 8>::new(pac.SPI0, (lcd_mosi, lcd_clk)); let spi = spi.init(     &mut pac.RESETS,     clocks.peripheral_clock.freq(),     40.MHz(), // SPI clock speed     embedded_hal::spi::MODE_0,  // SPI mode );`

### Powering the LCD

Ensure that the **VCC** pin on the LCD is connected to the **3.3V** pin on the RP2040, and the **GND** pin is connected to **GND** on the RP2040. These connections provide the required power to the LCD module.


## Using

To use this crate, your `Cargo.toml` file should contain:

```toml
waveshare_rp2040_plus_lcd_1_28 = "0.1.0"
```

In your program, you will need to call `waveshare_rp2040_lcd_1_28::Pins::new` to create
a new `Pins` structure. This will set up all the GPIOs for any on-board
devices. See the [examples](./examples) folder for more details.

## Examples

### General Instructions

To compile an example, clone the _rp-hal-boards_ repository and run:

```console
rp-hal-boards/boards/waveshare-rp2040-plus-lcd-1-28 $ cargo build --release --example <name>
```

You will get an ELF file called
`./target/thumbv6m-none-eabi/release/examples/<name>`, where the `target`
folder is located at the top of the _rp-hal-boards_ repository checkout. Normally
you would also need to specify `--target=thumbv6m-none-eabi` but when
building examples from this git repository, that is set as the default.

If you want to convert the ELF file to a UF2 and automatically copy it to the
USB drive exported by the RP2040 bootloader, simply boot your board into
bootloader mode and run:

```console
rp-hal-boards/boards/waveshare-rp2040-plus-lcd-1-28 $ cargo run --release --example <name>
```

If you get an error about not being able to find `elf2uf2-rs`, try:

```console
$ cargo install elf2uf2-rs, then repeating the `cargo run` command above.
```

### [waveshare_rp2040_plus_lcd_demo](./examples/waveshare_rp2040_plus_lcd_demo.rs)

Draws a red and green line with a blue regtangle.
After that is fills the screen line for line, that end it starts over with an
other colour, RED, GREEN and BLUE.

## Contributing

Contributions are what make the open source community such an amazing place to
be, learn, inspire, and create. Any contributions you make are **greatly
appreciated**.

The steps are:

1. Fork the Project by clicking the 'Fork' button at the top of the page.
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Make some changes to the code or documentation.
4. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
5. Push to the Feature Branch (`git push origin feature/AmazingFeature`)
6. Create a [New Pull Request](https://github.com/rp-rs/rp-hal-boards/pulls)
7. An admin will review the Pull Request and discuss any changes that may be required.
8. Once everyone is happy, the Pull Request can be merged by an admin, and your work is part of our project!

## Code of Conduct

Contribution to this crate is organized under the terms of the [Rust Code of
Conduct][CoC], and the maintainer of this crate, the [rp-rs team], promises
to intervene to uphold that code of conduct.

[CoC]: CODE_OF_CONDUCT.md
[rp-rs team]: https://github.com/orgs/rp-rs/teams/rp-rs

## License

The contents of this repository are dual-licensed under the _MIT OR Apache
2.0_ License. That means you can choose either the MIT license or the
Apache-2.0 license when you re-use this code. See `MIT` or `APACHE2.0` for more
information on each specific license.

Any submissions to this project (e.g. as Pull Requests) must be made available
under these terms.
