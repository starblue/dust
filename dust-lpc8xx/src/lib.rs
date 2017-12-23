#![feature(const_fn)]
#![feature(linkage)]
#![feature(used)]
#![no_std]

extern crate dust_cortex_m;
extern crate volatile_register;

#[cfg(feature = "lpc81x")]
pub mod lpc81x_vectors;
#[cfg(feature = "lpc82x")]
pub mod lpc82x_vectors;
#[cfg(feature = "lpc83x")]
pub mod lpc83x_vectors;
#[cfg(feature = "lpc84x")]
pub mod lpc84x_vectors;

pub mod syscon;
pub mod swm;
pub mod gpio;
pub mod usart;

pub const SWM: *mut swm::Swm = 0x4000_C000 as *mut swm::Swm;

pub const SYSCON: *mut syscon::Syscon = 0x4004_8000 as *mut syscon::Syscon;

pub const USART0: *mut usart::Usart = 0x4006_4000 as *mut usart::Usart;
pub const USART1: *mut usart::Usart = 0x4006_8000 as *mut usart::Usart;
pub const USART2: *mut usart::Usart = 0x4006_C000 as *mut usart::Usart;

pub const GPIO: *mut gpio::Gpio = 0xA000_0000 as *mut gpio::Gpio;
