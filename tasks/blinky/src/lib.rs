#![no_std]

use hal::*;

use embassy_executor::task;
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};
use embassy_time::Timer;
use embedded_hal::digital::OutputPin;

#[task]
pub async fn task_blinky(
    mut led: impl OutputPin + 'static,
    channel: &'static Channel<ThreadModeRawMutex, bool, 1>,
) {
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
        Timer::after_millis(500).await;
        led.set_low().ok();
        Timer::after_millis(500).await;
    }
}
