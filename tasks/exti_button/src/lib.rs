#![no_std]

use hal::*;

use embassy_executor::task;
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};
use embassy_time::Timer;
use embedded_hal::digital::InputPin;
use embedded_hal_async::digital::Wait;

#[task]
pub async fn task_button(
    mut input: impl InputPin + Wait + 'static,
    pullup: bool,
    channel: &'static Channel<ThreadModeRawMutex, bool, 1>,
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
