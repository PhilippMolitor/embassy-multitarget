#![no_std]
#![no_main]

use hal::*;

use embassy_rp::{
    clocks::{ClockConfig, RoscConfig},
    config::Config,
    gpio::{Level, Output},
    init,
};
use embassy_time::Timer;

use app::{run_app, AppConfig};

#[main]
async fn main(s: Spawner) {
    let config_clock = ClockConfig::default();
    let config = Config::new(config_clock);

    let p = init(Default::default());

    let mut p_led = Output::new(p.PIN_25, Level::Low);

    // TODO: fix this
    run_app(AppConfig { spawner: s, button_input: (), button_pullup: true, led_output: p_led });
}
