#![no_std]

pub extern crate defmt;
pub extern crate defmt_rtt;
pub extern crate embassy_executor;
pub extern crate embassy_sync;
pub extern crate embassy_time;
pub extern crate embassy_usb;
pub extern crate embedded_hal;
pub extern crate embedded_hal_async;
pub extern crate panic_probe;

#[cfg(feature = "arch-cortex-m")]
pub extern crate cortex_m;
#[cfg(feature = "arch-cortex-m")]
pub extern crate cortex_m_rt;

#[cfg(feature = "platform-stm32")]
pub extern crate embassy_stm32;

#[cfg(feature = "platform-rp")]
pub extern crate embassy_rp;
