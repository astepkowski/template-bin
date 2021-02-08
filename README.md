# Rust AVR executable template

A template for Rust based AVR executables.

**NOTE**: This software template repository is offered in the public domain. It is free to use, adapt, modify, distribute, with no restrictions and no crediting required.

Provides:

  * A Rust target specification JSON for ATmega328P - [`avr-atmega328p.json`](./avr-atmega328p.json)
  * A Rust target specification JSON for ATmega8 - [`avr-atmega8.json`](./avr-atmega8.json)
  * A GitHub-action based CI test pipeline

## Build instructions

Install Rust nightly.

```
rustup toolchain install nightly-2021-01-07
rustup component add rust-src --toolchain nightly-2021-01-07
```

Then run:

```
cargo +nightly-2021-01-07 build --target avr-atmega328p.json -Z build-std=core --release
```

The final ELF executable file will then be available at `target/avr-atmega328p/release/template-bin.elf`.

