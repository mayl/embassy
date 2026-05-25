#![no_std]
#![no_main]

// Calibration target: tight CPU busy-loop with LED1 (PA0) driven on. Companion
// to et_calib_busy_loop. The current delta between the two firmwares should
// match the LED's drive current (LED1 anode -> 3.3V, cathode -> R -> PA0;
// active-low — set_high() with set_inversion(true) drives the pin LOW).
//
// No embassy executor — same reasoning as et_calib_busy_loop.

use cortex_m_rt::entry;
use embassy_mspm0::Config;
use embassy_mspm0::gpio::{Level, Output};
// defmt-rtt is pulled in only to satisfy the defmt global-logger requirement
// from embassy-mspm0's "defmt" feature; nothing here actually logs.
use {defmt_rtt as _, panic_halt as _};

#[entry]
fn main() -> ! {
    let p = embassy_mspm0::init(Config::default());

    let mut led1 = Output::new(p.PA0, Level::Low);
    led1.set_inversion(true);
    led1.set_high(); // LED on

    // Keep the LED reference alive forever.
    core::mem::forget(led1);

    loop {
        cortex_m::asm::nop();
    }
}
