# [boardsource-blok] - Board Support for the [Blok]

You should include this crate if you are writing code that you want to run on
a [Blok] - an RP2040 based controller, made by [Boardsource],
built for the keyboard community. 
This crate includes the [rp2040-hal], but also configures each pin of the
RP2040 chip according to how it is connected up on the Blok.
More Information about the pin layout at [Peg].

[Blok]: https://boardsource.xyz/store/628b95b494dfa308a6581622
[boardsource-blok]: https://github.com/rp-rs/rp-hal-boards/tree/main/boards/boardsource-blok
[rp2040-hal]: https://github.com/rp-rs/rp-hal/tree/main/rp2040-hal
[Boardsource]: https://boardsource.xyz/
[Peg]: https://peg.software/docs/blok

## Using

To use this crate, your `Cargo.toml` file should contain:

```toml
boardsource-blok = "0.1.0"
```

In your program, you will need to call `blok::Pins::new` to create
a new `Pins` structure. This will set up all the GPIOs for any on-board
devices. See the [examples](./examples) folder for more details.

## Examples

### General Instructions

To compile an example, clone the _rp-hal-boards_ repository and run:

```console
rp-hal-boards/boards/boardsource-blok $ cargo build --release --example <name>
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
rp-hal-boards/boards/boardsource-blok $ cargo run --release --example <name>
```

If you get an error about not being able to find `elf2uf2-rs`, try:

```console
$ cargo install elf2uf2-rs
```
then try repeating the `cargo run` command above.

### From Scratch

To start a basic project from scratch, create a project using `cargo new project-name`. Within the
project directory, run `cargo add blok`, `cargo add cortex-m-rt`, and `cargo add panic-halt`. The
first command will add this HAL (Hardware Abstraction Layer), the second is required for the `#[entry]` macro, and _panic-halt_ creates a simple panic function, which just halts.

You'll also need to copy the cargo config file from the [repo](https://github.com/rp-rs/rp-hal-boards/blob/main/.cargo/config). It specifies the target and optimizing flags to the linker. You'll also need to copy [_memory.x_](https://github.com/rp-rs/rp-hal-boards/blob/main/memory.x) to your project root. This file tells the linker the flash and RAM layout, so it won't clobber the bootloader or write to an out of bounds memory address. 

The simplest working example, which does nothing except loop forever, is:

```ignore
#![no_std]
#![no_main]
use blok::entry;
use panic_halt as _;
#[entry]
fn see_doesnt_have_to_be_called_main() -> ! {
  loop {}
}
```

It can be placed in _/src/main.rs_. 

You can use `cargo run` to compile and install it. 
**Note**: You won't see any activity since this program does nothing. You can use the examples provided
to add more functionality. 
### [blok_rainbow](./examples/blok_rainbow.rs)

Runs a rainbow-effect color wheel on the on-board neopixel.

### [blok_reset_to_usb_boot](./examples/blok_reset_to_usb_boot.rs)

Resets the Blok after 10 seconds to usb boot mode.

### [blok_usb_keyboard_input](./examples/blok_usb_keyboard_input.rs)

Demonstrates emulating a USB Human Input Device (HID) Keyboard. The keyboard
will type "HELLO" five times.

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
2.0_ License. That means you can choose either the MIT license or the
Apache-2.0 license when you re-use this code. See `MIT` or `APACHE2.0` for more
information on each specific license.

Any submissions to this project (e.g. as Pull Requests) must be made available
under these terms.
