#!/bin/bash

# README.md
# https://github.com/Rahix/avr-hal/issues/124#issuecomment-766841616
cargo +nightly-2021-01-07 build --target avr-atmega8.json -Z build-std=core --release