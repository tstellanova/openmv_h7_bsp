/*
Copyright (c) 2020 Todd Stellanova
LICENSE: BSD3 (see LICENSE file)
*/

#![no_std]
#![no_main]


use panic_rtt_core::{self, rprintln, rtt_init_print};
use cortex_m_rt::{entry, exception, ExceptionFrame};

// use openmv_h7_bsp::peripherals;

use ehal::blocking::delay::DelayMs;
use ehal::digital::v2::OutputPin;
use ehal::digital::v2::ToggleableOutputPin;
use embedded_hal as ehal;


// // cortex-m-rt calls this for serious faults.  can set a breakpoint to debug
#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

#[entry]
fn main() -> ! {
    rtt_init_print!(NoBlockTrim);
    rprintln!("--> MAIN --");
    
    loop {

    }
}