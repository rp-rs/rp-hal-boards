# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

## 0.8.1 - 2024-10-05

### Added

- Ported PWM blink example from rp-pico
- Ported USB serial example from rp-pico
- Ported countdown blinky example from rp-pico
- Ported OLED display SSD1306 example from rp-pico (not tested with display)
- Ported PIO PWM example from rp-pico
- Ported interpolator example from rp-pico
- Ported GPIO in/out example from rp-pico
- Ported USB serial interrupt example from rp-pico
- Ported UART IRQ Buffer example from rp-pico
- Ported UART IRQ Echo example from rp-pico
- Ported USB twitchy mouse example from rp-pico
- Ported ws2812led example from rp-pico (not tested with actual LEDs)
- Ported SPI SD Card example from rp-pico (not tested, Failed to build : rust-lld: error: undefined symbol: _defmt_timestamp)
- Ported HD44780 diplay example from rp-pico (not tested with display)
- Ported I2C PIO example from rp-pico (not tested with sensor)
- Ported PWM servo example from rp-pico (not tested with servo)
- Ported rtic example from rp-pico
- Ported rtic monotonic example from rp-pico

## 0.8.0 - 2024-04-07

### Changed

- Update to rp2040-hal 0.10.0
- Update to embedded-hal 1.0.0

## 0.7.0 - 2023-09-02

### Changed

- Update to rp2040-hal 0.9.0

## 0.6.0 - 2023-02-18

### Changed

- Update to rp2040-hal 0.8.0

## 0.5.0 - 2022-12-11

### Changed

- Update to rp2040-hal 0.7.0

## 0.4.0 - 2022-08-26

### Changed

- Migrate from `embedded-time` to `fugit`
- Update to rp2040-hal 0.6.0

## 0.3.0 - 2022-06-13

### Changed

- Update to rp2040-hal 0.5.0

## 0.2.0 - 2022-03-11

### Changed

- Update to rp2040-hal 0.4.0

## 0.1.0 - 2021-12-20

- Initial release

