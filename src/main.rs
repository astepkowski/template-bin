#![no_std]
#![no_main]

use panic_halt as _;

{% case mcu_family -%}
{%- when "ATmega" -%}
#[atmega_hal::entry]
{%- when "ATtiny" -%}
#[attiny_hal::entry]
{%- endcase %}
fn main() -> ! {
    loop {}
}
