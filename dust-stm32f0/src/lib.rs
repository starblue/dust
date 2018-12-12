#![feature(const_fn)]
#![feature(linkage)]
#![no_std]

extern crate dust_cortex_m;

#[cfg(feature = "stm32f0x0")]
pub mod stm32f0x0_vectors;

pub mod rcc;
pub const RCC: *mut rcc::Rcc = 0x4002_1000 as *mut rcc::Rcc;

pub mod gpio;
pub const GPIOA: *mut gpio::Gpio = 0x4800_0000 as *mut gpio::Gpio;
pub const GPIOB: *mut gpio::Gpio = 0x4800_0400 as *mut gpio::Gpio;
pub const GPIOC: *mut gpio::Gpio = 0x4800_0800 as *mut gpio::Gpio;
pub const GPIOD: *mut gpio::Gpio = 0x4800_0c00 as *mut gpio::Gpio;
pub const GPIOF: *mut gpio::Gpio = 0x4800_1400 as *mut gpio::Gpio;

pub const GPIO: [Option<*mut gpio::Gpio>; 6] = [
    Some(GPIOA),
    Some(GPIOB),
    Some(GPIOC),
    Some(GPIOD),
    None,
    Some(GPIOF),
];
