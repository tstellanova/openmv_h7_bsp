/*
Copyright (c) 2020 Todd Stellanova
LICENSE: BSD3 (see LICENSE file)
*/

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_rtt_core::{self, rprintln, rtt_init_print};

use core::sync::atomic::{AtomicPtr, Ordering};
use ehal::blocking::delay::DelayMs;
use ehal::digital::v2::OutputPin;
use ehal::digital::v2::ToggleableOutputPin;
use embedded_hal as ehal;

use openmv_h7_bsp::board::Board;


// TODO trap DMA2_STR1 and DCMI interrupts

static mut BOARD_PTR: AtomicPtr<Board> = AtomicPtr::new(core::ptr::null_mut());

#[entry]
fn main() -> ! {
    rtt_init_print!(NoBlockTrim);
    rprintln!("--> MAIN --");

    let mut board = Board::default();
    // // this provides interrupt handlers access to the shared Board struct
    // unsafe {
    //     BOARD_PTR.store(&mut board, Ordering::SeqCst);
    // }

    let _ = board.rgb_leds.0.set_low();
    let _ = board.rgb_leds.1.set_high();
    let _ = board.rgb_leds.2.set_low();

    loop {
        for _ in 0..10 {
            for _ in 0..10 {
                let _ = board.rgb_leds.1.toggle();
                board.delay_source.delay_ms(25u32);
            }
            let _ = board.rgb_leds.0.toggle();
            board.delay_source.delay_ms(25u32);
        }
        let _ = board.rgb_leds.2.toggle();
        board.delay_source.delay_ms(50u32);
    }
}
