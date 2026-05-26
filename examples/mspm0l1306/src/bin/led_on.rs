#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_mspm0::Config;
use embassy_mspm0::gpio::{Level, Output};
use embassy_time::Timer;
use {defmt_rtt as _, panic_halt as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    info!("LED constant ON");
    let p = embassy_mspm0::init(Config::default());

    // Hold the LED in blinky's high-current "on" plateau. (Measured: the
    // set_high state is the ~1 mA LED-on level; set_low is the ~120 µA floor.)
    let mut led1 = Output::new(p.PA0, Level::Low);
    led1.set_inversion(true);
    led1.set_high();

    loop {
        Timer::after_secs(1).await;
    }
}
