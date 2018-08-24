#![feature(asm)]
#![feature(linkage)]
#![feature(used)]
#![no_std]

#[macro_use]
extern crate dust;
extern crate dust_runtime;
extern crate volatile_register;

use dust_runtime::crt0;

#[cfg(any(feature = "cortex-m0", feature = "cortex-m0plus", feature = "cortex-m23"))]
pub mod cm0_vectors;
#[cfg(any(feature = "cortex-m3", feature = "cortex-m4", feature = "cortex-m7"))]
pub mod cm3_vectors;

pub mod intrinsics;

pub mod nvic;
pub mod scb;
pub mod sys_tick;

pub const SYS_TICK: *mut sys_tick::SysTick = 0xE000_E010 as *mut sys_tick::SysTick;
pub const NVIC: *mut nvic::Nvic = 0xE000_E100 as *mut nvic::Nvic;
pub const SCB: *mut scb::Scb = 0xE000_ED00 as *mut scb::Scb;

unsafe fn reset_handler() {
    crt0();
}
