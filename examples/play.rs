/*
Copyright (c) 2020 Todd Stellanova
LICENSE: BSD3 (see LICENSE file)
*/

#![no_std]
#![no_main]


use panic_rtt_core::{self, rprintln, rtt_init_print};
use cortex_m_rt::{entry, exception, ExceptionFrame};

use openmv_h7_bsp::peripherals;

use ehal::blocking::delay::DelayMs;
use ehal::digital::v2::OutputPin;
use ehal::digital::v2::ToggleableOutputPin;
use embedded_hal as ehal;



#[entry]
fn main() -> ! {
    rtt_init_print!(NoBlockTrim);
    rprintln!("--> MAIN --");

    let     (
        mut rgb_leds,
        _ir_led,
        mut delay_source,
        _dcmi_data_pins,

    )
        = peripherals::setup();

    loop {
        for _ in 0..10 {
            for _ in 0..10 {
                let _ = rgb_leds.0.toggle();
                delay_source.delay_ms(125u32);
            }
            let _ = rgb_leds.1.toggle();
            delay_source.delay_ms(250u32);
        }
        let _ = rgb_leds.2.toggle();
        delay_source.delay_ms(500u32);
    }
}