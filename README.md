# Rust AVR executable template

A template for Rust based AVR executables. Use with cargo-generate:

```
  cargo generate --git https://github.com/astepkowski/rust-avr-template.git
```
then select AVR family (ATmega, ATtiny) and chip.

**NOTE**: This software template repository is offered in the public domain. It is free to use, adapt, modify, distribute, with no restrictions and no crediting required.

## Build instructions

Install Rust nightly.

```
rustup toolchain install nightly-2022-06-13
rustup component add rust-src --toolchain nightly-2022-06-13
```

Then run:

```
cargo build --release
```

The final ELF executable file will then be available at `target/avr-<mcu_name>/release/template-bin.elf`.

