#![no_std]

#[cfg(not(any(unix, windows)))]
use core::panic::PanicInfo;

#[cfg(not(any(unix, windows)))]
#[panic_handler]
pub fn panic_handler(_: &PanicInfo) -> ! {
    loop {}
}
