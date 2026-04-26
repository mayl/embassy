#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_mspm0::Config;
use embassy_mspm0::gpio::{Input, Level, Output, Pull};
use embassy_time::Timer;
use {defmt_rtt as _, panic_halt as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    info!("sleep demo start");
    let p = embassy_mspm0::init(Config::default());

    let mut led = Output::new(p.PA0, Level::Low);
    led.set_inversion(true);
    let button = Input::new(p.PA14, Pull::Up);

    loop {
        led.set_high();
        Timer::after_millis(100).await;
        led.set_low();
        Timer::after_millis(900).await;
        if button.is_low() {
            info!("button pressed");
        }
    }
}
