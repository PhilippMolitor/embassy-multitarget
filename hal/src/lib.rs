#![no_std]
// for custom panic handlers
#![allow(internal_features)]
#![feature(core_intrinsics)]

// arch support

#[cfg(feature = "_arch-cortex-m")]
pub use cortex_m;
#[cfg(feature = "_arch-cortex-m")]
pub use cortex_m_rt;

// embassy

pub use embassy_executor;
pub use embassy_sync;
pub use embassy_time;
pub use embassy_usb;
// the embassy-executor macros and types are re-exported directly for convenience
pub use embassy_executor::{main, task, Spawner};

// platform: Raspberry Pi

#[cfg(feature = "_platform-rp")]
pub use embassy_rp;
#[cfg(feature = "_platform-rp")]
pub use pio;
#[cfg(feature = "_platform-rp")]
pub use pio_proc;

// platform: STM32

#[cfg(feature = "_platform-stm32")]
pub use embassy_stm32;

// platform: CH32

#[cfg(feature = "_platform-ch32")]
pub use qingke;
#[cfg(feature = "_platform-ch32")]
pub use qingke_rt;
#[cfg(feature = "_platform-ch32")]
pub use ch32_hal;

// embedded-hal

pub use embedded_hal;
pub use embedded_hal_async;
pub use embedded_io;
pub use embedded_io_async;

// debugging utilities

pub use defmt;
#[allow(unused_imports)]
use defmt_rtt as _;

// panic handling

mod hal_panic;
#[allow(unused_imports)]
use hal_panic as _;
