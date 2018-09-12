#![feature(const_fn)]
#![feature(linkage)]
#![no_std]

extern crate dust_cortex_m;

#[cfg(feature = "lpc13xx")]
pub mod lpc13xx_vectors;

pub mod gpio;
pub mod iocon;
pub mod syscon;
pub mod uart;

pub const SYSCON: *mut syscon::Syscon = 0x4004_8000 as *mut syscon::Syscon;

pub const IOCON: *mut iocon::Iocon = 0x4004_4000 as *mut iocon::Iocon;

pub const GPIO: [*mut gpio::Gpio; 4] = [
    0x5000_0000 as *mut gpio::Gpio,
    0x5001_0000 as *mut gpio::Gpio,
    0x5002_0000 as *mut gpio::Gpio,
    0x5003_0000 as *mut gpio::Gpio,
];

pub const UART: *mut uart::Uart = 0x4000_8000 as *mut uart::Uart;
