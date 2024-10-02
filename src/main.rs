#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let mut led = Output::new(p.PE10, Level::High, Speed::Low);

    loop {
        led.set_high();
        info!("high");
        Timer::after_millis(1000).await;

        led.set_low();
        info!("low");
        Timer::after_millis(1000).await;
    }
}