// debug: use panic-probe for panic handling.
//        this logs panics to the host over RTT
//        note: panic-probe only works with cortex-m

#[cfg(all(debug_assertions, feature = "_arch-cortex-m"))]
use panic_probe as _;

// release: use core::intrinsics::abort for panic handling

#[cfg(not(debug_assertions))]
use core::{intrinsics, panic::PanicInfo};

#[cfg(not(debug_assertions))]
#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    intrinsics::abort()
}
