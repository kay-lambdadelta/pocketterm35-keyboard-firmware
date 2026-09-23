## PocketTerm35 Keyboard Firmware

This implements a replacement for the RP2040 keyboard/misc controller for the [PocketTerm35](https://www.waveshare.com/pocketterm35.htm), based upon the work [here](https://github.com/black-ghost-off/pocketTerm35-keyboard-firmware)

## Compiling

To compile, you need a nightly rust toolchain (easily installable via [`rustup`](https://rustup.rs/)) and an appropriate C toolchain for the RP2040.

| Distro | Development Package Name   |
| ------ | -------------------------- |
| Debian | `rustup gcc-arm-none-eabi` |

Run

```bash
cargo build --release
```

Your firmware will be put at `target/thumbv6m-none-eabi/release/pocketterm35-keyboard-firmware.elf`.

## Flashing

On your device, flash via `picotool` or your favorite flashing utility.

```bash
picotool load target/thumbv6m-none-eabi/release/pocketterm35-keyboard-firmware.elf
```
