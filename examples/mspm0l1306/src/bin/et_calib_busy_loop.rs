#![no_std]
#![no_main]

// Calibration target: tight CPU busy-loop, all GPIO left in reset state (input,
// no pull). Measures MSPM0L1306 RUN-mode current under a fixed CPU load with
// no LED, peripheral, or timer activity beyond what embassy_mspm0::init does.
//
// No embassy executor — using cortex_m_rt::entry directly so there is no
// chance of an inadvertent WFE dropping the chip into SLEEP.

use cortex_m_rt::entry;
use embassy_mspm0::Config;
// defmt-rtt is pulled in only to satisfy the defmt global-logger requirement
// from embassy-mspm0's "defmt" feature; nothing here actually logs.
use {defmt_rtt as _, panic_halt as _};

#[entry]
fn main() -> ! {
    let _p = embassy_mspm0::init(Config::default());
    loop {
        cortex_m::asm::nop();
    }
}
