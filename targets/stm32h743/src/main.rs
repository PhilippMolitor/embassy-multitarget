#![no_std]
#![no_main]

use hal::*;

use embassy_executor::{main, Spawner};

// Doeesn't work
#[main]
async fn main(_s: Spawner) {
    loop {}
}

/*
use hal::prelude::*;

use defmt::*;
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::{main, task, Spawner};
use embassy_stm32::{
    gpio::{Level, Output, Pin, Speed},
    init, Config,
};
use embassy_time::Timer;

#[embassy_executor::main]
async fn main(s: Spawner) {
    let p = init(Config::default());

    info!("Hello World!");

    s.spawn(task_blink(p.PB14)).expect("task_blink spawn failed");
}

#[task]
async fn task_blink(pin: impl Pin) {
    let mut led = Output::new(pin, Level::High, Speed::Low);

    loop {
        info!("high");
        led.set_high();
        Timer::after_millis(500).await;

        info!("low");
        led.set_low();
        Timer::after_millis(500).await;
    }
}
*/
