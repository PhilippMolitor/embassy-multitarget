#![no_std]
#![no_main]

use hal::*;

use defmt::*;

use embassy_executor::{main, Spawner};
use embassy_stm32::{
    exti::ExtiInput,
    gpio::{Level, Output, Pull, Speed},
    init,
    rcc::{Hse, HseMode, LsConfig, Sysclk},
    time::mhz,
    Config,
};

use app::{run_app, AppConfig};

#[main]
async fn main(s: Spawner) {
    let mut config = Config::default();

    // clock configuration
    config.rcc.hse = Some(Hse { freq: mhz(25), mode: HseMode::Oscillator });
    config.rcc.sys = Sysclk::HSE;
    config.rcc.ls = LsConfig::default_lse();

    let p = init(config);

    info!("Hello World!");

    let p_led = Output::new(p.PE3, Level::High, Speed::Low);
    let p_button = ExtiInput::new(p.PC13, p.EXTI13, Pull::Down);

    run_app(AppConfig {
        spawner: s,
        button_input: p_button,
        button_pullup: false,
        led_output: p_led,
    });
}
