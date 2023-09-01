# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

## 0.8.0 - 2023-09-02

### Changed

- Update to rp2040-hal 0.9.0
- Update to ws2812-pio 0.7.0
- Update to i2c-pio 0.7.0

## 0.7.0 - 2023-02-18

### Changed

- Update to rp2040-hal 0.8.0
- Update to ws2812-pio 0.6.0
- Update to i2c-pio 0.6.0
- Update to embedded-sdmmc 0.4.0

## 0.6.0 - 2022-12-11

### Changed

- Update to rp2040-hal 0.7.0
- Update to ws2812-pio 0.5.0
- Update to i2c-pio 0.5.0

## 0.5.0 - 2022-08-26

### Added

- `rp2040-e5` feature enabling the workaround for errata 5 on the USB device peripheral.
- Support for critical-section 1.0.0 in the examples.
- Example for the interpolator

### Changed

- Use `rp2040-hal`'s entry function.
- Migrate from `embedded-time` to `fugit`
- Bump `ws2812-pio` to 0.4.0
- Bump `i2c-pio` to 0.4.0
- Update to rp2040-hal 0.6.0

### Removed

- Unused dependencies

## 0.4.0 - 2022-06-13

### Changed

- Update to rp2040-hal 0.5.0

## 0.3.0 - 2022-03-11

### Changed

- Update to rp-hal 0.4.0

## 0.2.0 - 2021-12-23

### Added

- Lots of things!

### Changed

- Basically re-written.

## 0.1.3 - 2021-02-03

- Last release outside the [rp-rs] organisation by [@jannic].

[@jannic]: https://github.com/jannic
[rp-rs]: https://github.com/rp-rs
