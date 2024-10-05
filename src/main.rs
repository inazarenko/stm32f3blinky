#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{
    exti::ExtiInput,
    gpio::{Level, Output, Pin, Pull, Speed},
};
use embassy_time::{with_timeout, Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

async fn play_circle(leds: &mut [Output<'_>], btn: &mut ExtiInput<'_>) {
    loop {
        for led in leds.iter_mut() {
            led.toggle();
            if let Ok(_) =
                with_timeout(Duration::from_millis(1000), btn.wait_for_falling_edge()).await
            {
                info!("button pressed");
                return;
            }
        }
        info!("circle done");
    }
}

#[allow(dead_code)]
fn fast_config() -> embassy_stm32::Config {
    let mut cfg = embassy_stm32::Config::default();
    cfg.rcc.hse = Some(embassy_stm32::rcc::Hse {
        freq: embassy_stm32::time::Hertz(8_000_000),
        mode: embassy_stm32::rcc::HseMode::Bypass,
    });
    cfg.rcc.apb1_pre = embassy_stm32::rcc::APBPrescaler::DIV2;
    cfg.rcc.sys = embassy_stm32::rcc::Sysclk::PLL1_P;
    cfg.rcc.pll = Some(embassy_stm32::rcc::Pll {
        src: embassy_stm32::rcc::PllSource::HSE,
        prediv: embassy_stm32::rcc::PllPreDiv::DIV1,
        mul: embassy_stm32::rcc::PllMul::MUL9,
    });
    cfg
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(embassy_stm32::Config::default());

    let mut cps = cortex_m::peripheral::Peripherals::take().unwrap();
    cps.DCB.enable_trace();
    cps.DWT.enable_cycle_counter();
    let before = cortex_m::peripheral::DWT::cycle_count();
    Timer::after_millis(100).await;
    let after = cortex_m::peripheral::DWT::cycle_count();
    info!("Mcycles/s: {}", (after - before + 49_999) / 100_000);

    let led_pins = [
        p.PE9.degrade(),
        p.PE10.degrade(),
        p.PE11.degrade(),
        p.PE12.degrade(),
        p.PE13.degrade(),
        p.PE14.degrade(),
        p.PE15.degrade(),
        p.PE8.degrade(),
    ];
    let mut leds = led_pins.map(|really| Output::new(really, Level::Low, Speed::Low));
    let mut btn = ExtiInput::new(p.PA0, p.EXTI0, Pull::Down);

    loop {
        play_circle(&mut leds, &mut btn).await;
        for led in leds.iter_mut() {
            led.set_low();
        }
        btn.wait_for_falling_edge().await;
    }
}
