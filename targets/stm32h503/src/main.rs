#![no_std]
#![no_main]

use hal::*;

#[allow(unused_imports)]
use {defmt_rtt as _, panic_probe as _};

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
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};

use blinky::task_blinky;
use exti_button::task_button;

static CHANNEL_BLINKY_CONTROL: Channel<ThreadModeRawMutex, bool, 1> = Channel::new();

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

    s.spawn(task_blinky(p_led, &CHANNEL_BLINKY_CONTROL)).expect("blink spawn failed");
    s.spawn(task_button(p_button, true, &CHANNEL_BLINKY_CONTROL)).expect("button spawn failed");
}
