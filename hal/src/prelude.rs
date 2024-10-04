pub use defmt;
pub use defmt_rtt;
pub use embassy_executor;
pub use embassy_sync;
pub use embassy_time;
pub use embassy_usb;
pub use embedded_hal;
pub use embedded_hal_async;
pub use panic_probe;

#[cfg(feature = "arch-cortex-m")]
pub use cortex_m;
#[cfg(feature = "arch-cortex-m")]
pub use cortex_m_rt;

#[cfg(feature = "platform-stm32")]
pub use embassy_stm32;

#[cfg(feature = "platform-rp")]
pub use embassy_rp;
