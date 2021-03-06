//! bare2.rs
//!
//! Measuring execution time
//!
//! What it covers
//! - Generating documentation
//! - Using core peripherals
//! - Measuring time using the DWT
//! - ITM tracing
//!

#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m::{iprintln, peripheral::DWT, Peripherals};
use cortex_m_rt::entry;

// burns CPU cycles by just looping `i` times
#[inline(never)]
fn wait(i: u32) {
    for _ in 0..i {
        // no operation (ensured not optimized out)
        cortex_m::asm::nop();
    }
}

#[entry]
fn main() -> ! {
    let mut p = Peripherals::take().unwrap();
    let stim = &mut p.ITM.stim[0];
    let mut dwt = p.DWT;

    iprintln!(stim, "bare2");

    dwt.enable_cycle_counter();

    // Reading the cycle counter can be done without `owning` access
    // the DWT (since it has no side effetc).
    //
    // Look in the docs:
    // pub fn enable_cycle_counter(&mut self)
    // pub fn get_cycle_count() -> u32
    //
    // Notice the difference in the function signature!

    let start = DWT::get_cycle_count();
    wait(1_000_000);
    let end = DWT::get_cycle_count();

    // notice all printing outside of the section to measure!
    iprintln!(stim, "Start {:?}", start);
    iprintln!(stim, "End {:?}", end);

    loop {}
}

// 0. Setup
//    > cargo doc --open
//    This will document your crate, and open the docs in your browser.
//    If it does not auto-open, then copy paste the path in your browser.
//
//    In the docs, seach (`S`) for DWT, and click `cortex_m::peripheral::DWT`.
//    Read the API docs.
//
// 1. Build and run the application (debug build).
//
//    > cargo build --example bare2
//    (or use the vscode build task)
//
//    What is the output in the ITM console?
//
//    ** your answer here **
//      [2019-03-03T13:51:36.839Z]   bare2
//      [2019-03-03T13:51:37.144Z]   Start 1933161631
//      [2019-03-03T13:51:37.145Z]   End 1937161639

//
//    Compute the ratio between debug/release optimized code
//    (the speedup).
//
//    ** your answer here **
//    for the debug, the time was 623 000 207
//    for the release it was 4 000 008
//    The release is approximately 150x times faster
//
// commit your answers (bare2_1)
//
// 3. *Optional
//    Inspect the generated binaries, and try stepping through the code
//    for both debug and release binaries. How do they differ?
//
//    ** your answer here **
//
//    commit your answers (bare2_2)
