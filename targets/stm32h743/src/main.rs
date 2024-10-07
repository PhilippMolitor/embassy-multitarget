#![no_std]
#![no_main]

use hal::*;

#[allow(unused_imports)]
use {defmt_rtt as _, panic_probe as _};

use defmt::*;

use embassy_executor::{main, task, Spawner};
use embassy_stm32::{
    exti::ExtiInput,
    gpio::{Level, Output, Pull, Speed},
    init,
    rcc::{Hse, HseMode, LsConfig, Sysclk},
    time::mhz,
    Config,
};
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};
use embassy_time::Timer;
use embedded_hal::digital::{InputPin, OutputPin};
use embedded_hal_async::digital::Wait;

type BlinkyControlChannel = Channel<ThreadModeRawMutex, bool, 1>;
static CHANNEL_BLINKY_CONTROL: BlinkyControlChannel = Channel::new();

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

    s.spawn(task_blink(s, p_led)).expect("blink spawn failed");
    s.spawn(task_button(s, p_button)).expect("button spawn failed");
}

#[task]
async fn task_blink(_s: Spawner, mut led: impl OutputPin + 'static) {
    let mut active = true;

    loop {
        match CHANNEL_BLINKY_CONTROL.try_receive() {
            Ok(_) => active = !active,
            Err(_) => (),
        }

        if !active {
            Timer::after_millis(100).await;
            continue;
        }

        // do the blinky
        led.set_high().ok();
        Timer::after_millis(500).await;
        led.set_low().ok();
        Timer::after_millis(500).await;
    }
}

#[task]
async fn task_button(_s: Spawner, mut input: impl InputPin + Wait + 'static) {
    const DEBOUNCE_DELAY: u64 = 50;

    #[inline(always)]
    async fn debounce() {
        Timer::after_millis(DEBOUNCE_DELAY).await;
    }

    loop {
        input.wait_for_rising_edge().await.unwrap();
        debounce().await;

        if input.is_high().unwrap() {
            info!("Pressed!");
        } else {
            continue; // debounce failed
        }

        input.wait_for_falling_edge().await.unwrap();
        debounce().await;

        if input.is_low().unwrap() {
            info!("Released!");
            CHANNEL_BLINKY_CONTROL.send(true).await;
        } else {
            continue; // debounce failed
        }
    }
}
