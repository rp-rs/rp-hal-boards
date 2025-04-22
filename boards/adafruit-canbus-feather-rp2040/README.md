# [adafruit-canbus-feather-rp2040] - Board Support for the [Adafruit CANBus Feather RP2040]

You should include this crate if you are writing code that you want to run on
an [Adafruit CANBus Feather RP2040] - a Feather form-factor RP2040 board from
Adafruit with built in CAN Bus support.

This crate includes the [rp2040-hal], but also configures each pin of the RP2040
chip according to how it is connected on the Feather.

The board's CAN header is wired up to an [MCP25625] CAN controller which
includes an integrated transceiver. That device is made available to you on the
SPI1 bus. See src/lib.rs and the [Adafruit docs] for complete pinout, and check
the examples for a simple send/receive on 500k CAN.

[Adafruit CANBus Feather RP2040]: https://www.adafruit.com/product/5724
[adafruit-canbus-feather-rp2040]: https://github.com/rp-rs/rp-hal-boards/tree/main/boards/adafruit-canbus-feather-rp2040
[rp2040-hal]: https://github.com/rp-rs/rp-hal/tree/main/rp2040-hal
[MCP25625]: https://ww1.microchip.com/downloads/aemDocuments/documents/OTH/ProductDocuments/DataSheets/MCP25625-CAN-Controller-Data-Sheet-20005282C.pdf
[Adafruit docs]: https://learn.adafruit.com/adafruit-rp2040-can-bus-feather/pinouts

## Using

To use this crate, your `Cargo.toml` file should contain:

```toml
adafruit-canbus-feather-rp2040 = "0.1.0"
```

In your program, you will need to call `adafruit_canbus_feather_rp2040::Pins::new` to create
a new `Pins` structure. This will set up all the GPIOs for any on-board
devices. See the [examples](./examples) folder for more details.

## Examples

### General Instructions

To compile an example, clone the _rp-hal-boards_ repository and run:

```console
rp-hal-boards/boards/adafruit-canbus-feather-rp2040 $ cargo build --release --example <name>
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
rp-hal-boards/boards/adafruit-canbus-feather-rp2040 $ cargo run --release --example <name>
```

If you get an error about not being able to find `elf2uf2-rs`, try:

```console
$ cargo install elf2uf2-rs, then repeating the `cargo run` command above.
```

### [adafruit_canbus_feather_blinky](./examples/adafruit_canbus_feather_blinky.rs)

Flashes the Feather's onboard LED on and off.

### [adafruit_canbus_feather_neopixel_rainbow](./examples/adafruit_canbus_feather_neopixel_rainbow.rs)

Flows smoothly through various colors on the Feather's onboard NeoPixel LED.

### [adafruit_canbus_feather_can](./examples/adafruit_canbus_feather_can.rs)

Flashes onboard LED and transmits on MCP25625 CAN once per second. LED goes
solid upon receipt of any CAN message.

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
