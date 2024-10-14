#![no_std]

use hal::*;

use defmt::*;

use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};
use embassy_time::Timer;
use embedded_hal::digital::{InputPin, OutputPin};
use embedded_hal_async::digital::Wait;

pub struct AppConfig<EXTIINPUT, OUTPUTPIN>
where
    EXTIINPUT: InputPin + Wait + 'static,
    OUTPUTPIN: OutputPin + 'static,
{
    pub spawner: Spawner,
    pub button_input: EXTIINPUT,
    pub button_pullup: bool,
    pub led_output: OUTPUTPIN,
}

type BlinkyChannel = Channel<ThreadModeRawMutex, bool, 1>;
static CHANNEL_BLINKY_CONTROL: BlinkyChannel = Channel::new();

#[inline(always)]
pub fn run_app<EXTIINPUT, OUTPUTPIN>(config: AppConfig<EXTIINPUT, OUTPUTPIN>)
where
    EXTIINPUT: InputPin + Wait + 'static,
    OUTPUTPIN: OutputPin + 'static,
{
    debug!("booting app...");

    #[cfg(feature = "blinky")]
    config
        .spawner
        .spawn(task_blinky(config.led_output, &CHANNEL_BLINKY_CONTROL))
        .expect("blinky spawn failed");

    #[cfg(feature = "button")]
    config
        .spawner
        .spawn(task_button(config.button_input, config.button_pullup, &CHANNEL_BLINKY_CONTROL))
        .expect("button spawn failed");

    debug!("app tasks spawned");
}

#[cfg(feature = "blinky")]
#[task]
pub async fn task_blinky(mut led: impl OutputPin + 'static, channel: &'static BlinkyChannel) {
    const BLINKY_DELAY: u64 = if cfg!(feature = "blinky-slow") { 1000 } else { 500 };

    let mut active = true;

    loop {
        match channel.try_receive() {
            Ok(_) => active = !active,
            Err(_) => (),
        }

        if !active {
            Timer::after_millis(100).await;
            continue;
        }

        // do the blinky
        led.set_high().ok();
        Timer::after_millis(BLINKY_DELAY).await;
        led.set_low().ok();
        Timer::after_millis(BLINKY_DELAY).await;
    }
}

#[cfg(feature = "button")]
#[task]
pub async fn task_button(
    mut input: impl InputPin + Wait + 'static,
    pullup: bool,
    channel: &'static BlinkyChannel,
) {
    const DEBOUNCE_DELAY: u64 = 50;

    #[inline(always)]
    async fn debounce() {
        Timer::after_millis(DEBOUNCE_DELAY).await;
    }

    if pullup {
        loop {
            input.wait_for_falling_edge().await.unwrap();
            debounce().await;

            if !input.is_low().unwrap() {
                continue; // debounce failed
            }

            input.wait_for_rising_edge().await.unwrap();
            debounce().await;

            if input.is_high().unwrap() {
                channel.send(true).await;
            } else {
                continue; // debounce failed
            }
        }
    } else {
        loop {
            input.wait_for_rising_edge().await.unwrap();
            debounce().await;

            if !input.is_high().unwrap() {
                continue; // debounce failed
            }

            input.wait_for_falling_edge().await.unwrap();
            debounce().await;

            if input.is_low().unwrap() {
                channel.send(true).await;
            } else {
                continue; // debounce failed
            }
        }
    }
}
