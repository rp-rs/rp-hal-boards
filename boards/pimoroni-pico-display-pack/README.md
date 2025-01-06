# [pimoroni_pico_display_pack] - Board Support for the [Pimoroni Pico Display Pack] 2.8"

You should include this crate if you are writing code that you want to run on
a [Pimoroni Pico Display Pack] - a 2.8" IPS display add on board for the Raspberry Pi Pico. This includes the buttons, the display, as well as the QW/ST connector (using a i2c protocol).

This crate includes the [rp2040-hal], but also configures each pin of the
RP2040 chip according to how it is connected up on the Pico Display Pack.

[Pimoroni Pico Display Pack]: https://shop.pimoroni.com/products/pico-display-pack-2-8
[pimoroni_display_pack]: https://github.com/rp-rs/rp-hal-boards/tree/main/boards/pimoroni_pico-display-pack
[rp2040-hal]: https://github.com/rp-rs/rp-hal/tree/main/rp2040-hal
[Raspberry Silicon RP2040]: https://www.raspberrypi.org/products/rp2040/

## Using

To use this crate, your `Cargo.toml` file should contain:

```toml
pimoroni-pico-display-pack = "0.1.0"
```

In your program, you will need to call `pimoroni_pico_display_pack::PicoDisplayPack::new` to create
a new `PicoDisplayPack` structure. This will set up all the GPIOs for any on-board
devices and configure common clocks. See the [examples](./examples) folder for more details.

## Examples

### General Instructions

To compile an example, clone the _rp-hal-boards_ repository and run:

```console
rp-hal-boards/boards/pimoroni_pico_display_pack $ cargo build --release --example <name>
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
rp-hal-boards/boards/pimoroni_pico_display_pack $ cargo run --release --example <name>
```

If you get an error about not being able to find `elf2uf2-rs`, try:

```console
$ cargo install elf2uf2-rs
```
then try repeating the `cargo run` command above.

### [showcase](./examples/showcase.rs)

Showcases both the LCD display, button presses, RGB LED blinking, as well as the QW/ST connector using a SPG30 sensor.

## Contributing

Contributions are what make the open source community such an amazing place to
be learn, inspire, and create. Any contributions you make are **greatly
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
2.0_ License. That means you can chose either the MIT licence or the
Apache-2.0 licence when you re-use this code. See `MIT` or `APACHE2.0` for more
information on each specific licence.

Any submissions to this project (e.g. as Pull Requests) must be made available
under these terms.
