#![feature(linkage)]
#![no_std]

extern crate dust_cortex_m;

#[cfg(feature = "lpc802")]
pub mod lpc802_vectors;
#[cfg(feature = "lpc804")]
pub mod lpc804_vectors;
#[cfg(feature = "lpc81x")]
pub mod lpc81x_vectors;
#[cfg(feature = "lpc82x")]
pub mod lpc82x_vectors;
#[cfg(feature = "lpc83x")]
pub mod lpc83x_vectors;
#[cfg(feature = "lpc84x")]
pub mod lpc84x_vectors;

pub mod gpio;
#[cfg(feature = "lpc804")]
pub mod plu;
pub mod swm;
pub mod syscon;
pub mod usart;

pub const SWM: *mut swm::Swm = 0x4000_C000 as *mut swm::Swm;

pub const SYSCON: *mut syscon::Syscon = 0x4004_8000 as *mut syscon::Syscon;

#[cfg(any(feature = "lpc802", feature = "lpc804"))]
pub const USART: [usart::Usart; 2] = [usart::Usart::at(0x4006_4000), usart::Usart::at(0x4006_8000)];

#[cfg(any(feature = "lpc81x", feature = "lpc82x"))]
pub const USART: [usart::Usart; 3] = [
    usart::Usart::at(0x4006_4000),
    usart::Usart::at(0x4006_8000),
    usart::Usart::at(0x4006_C000),
];

#[cfg(feature = "lpc83x")]
pub const USART: [usart::Usart; 1] = [usart::Usart::at(0x4006_4000)];

#[cfg(feature = "lpc84x")]
pub const USART: [usart::Usart; 5] = [
    usart::Usart::at(0x4006_4000),
    usart::Usart::at(0x4006_8000),
    usart::Usart::at(0x4006_C000),
    usart::Usart::at(0x4007_0000),
    usart::Usart::at(0x4007_4000),
];

pub const GPIO: *mut gpio::Gpio = 0xA000_0000 as *mut gpio::Gpio;

#[cfg(feature = "lpc804")]
pub const PLU: *mut plu::Plu = 0x4002_8000 as *mut plu::Plu;
