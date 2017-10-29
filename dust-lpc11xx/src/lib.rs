#![feature(const_fn)]
#![feature(lang_items)]
#![feature(linkage)]
#![feature(used)]
#![no_std]

extern crate volatile_register;

#[cfg(not(target_os = "linux"))]
pub mod runtime;

#[cfg(not(target_os = "linux"))]
pub mod cm0_vectors;

#[cfg(feature = "lpc11xx")]
pub mod lpc11xx_vectors;

pub mod syscon;
pub mod iocon;
pub mod gpio;
pub mod uart;

pub const SYSCON: *mut syscon::Syscon = 0x4004_8000 as *mut syscon::Syscon;

pub const IOCON: *mut iocon::Iocon = 0x4004_4000 as *mut iocon::Iocon;

pub const GPIO0: *mut gpio::Gpio = 0x5000_0000 as *mut gpio::Gpio;
pub const GPIO1: *mut gpio::Gpio = 0x5001_0000 as *mut gpio::Gpio;
pub const GPIO2: *mut gpio::Gpio = 0x5002_0000 as *mut gpio::Gpio;
pub const GPIO3: *mut gpio::Gpio = 0x5003_0000 as *mut gpio::Gpio;

pub const UART: *mut uart::Uart = 0x4000_8000 as *mut uart::Uart;

#[cfg(not(target_os = "linux"))]
extern "C" {
    fn main();
}

#[cfg(not(target_os = "linux"))]
fn reset_handler() {
    unsafe {
        main();
    }
}
