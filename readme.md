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

### Notes

This works like the original firmware, and as you would expect a keyboard to work.

Holding Start+Select (for 4 seconds) swaps the keyboard mode, from Keyboard to Gamepad to Mouse emulation, cycling on the fourth hold. The indicator LED on the keyboard will flash to indicate the switch occurred.

If the firmware crashes, the indicator LED will spam repeatedly your error in morse code (I found this [webapp](https://sollozzo2.github.io/smorse/) to work reliably). Please file a bug report if it crashes, this should never ever occur.
