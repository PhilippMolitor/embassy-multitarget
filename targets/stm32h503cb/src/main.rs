#![no_std]
#![no_main]

use hal::*;

use defmt::*;

use embassy_executor::{main, Spawner};
use embassy_stm32::{
    exti::ExtiInput,
    gpio::{Level, Output, Pull, Speed},
    init,
    rcc::{Hse, HseMode, LsConfig},
    time::mhz,
    Config,
};

use app::{run_app, AppConfig};

#[main]
async fn main(s: Spawner) {
    let mut config = Config::default();

    // clock configuration
    config.rcc.hse = Some(Hse { freq: mhz(8), mode: HseMode::Oscillator });
    // config.rcc.sys = Sysclk::HSE;
    config.rcc.ls = LsConfig::default_lse();

    let p = init(config);

    info!("Hello World!");

    let p_led = Output::new(p.PC13, Level::High, Speed::Low);
    let p_button = ExtiInput::new(p.PA0, p.EXTI0, Pull::Up);

    run_app(AppConfig {
        spawner: s,
        button_input: p_button,
        button_pullup: true,
        led_output: p_led,
    });
}
