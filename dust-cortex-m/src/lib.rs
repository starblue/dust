#![feature(linkage)]
#![no_std]

use dust_runtime::crt0;

#[cfg(feature = "v6m")]
pub mod v6m_vectors;
#[cfg(feature = "v7m")]
pub mod v7m_vectors;

#[cfg(feature = "v8m_base")]
pub mod v8m_base_vectors;
#[cfg(feature = "v8m_main")]
pub mod v8m_main_vectors;

pub mod intrinsics;

pub mod nvic;
pub mod scb;
pub mod sys_tick;

pub const EXC_RETURN_USE_PSP: u32 = 1 << 2;
pub const EXC_RETURN_THREAD_MODE: u32 = 1 << 3;

pub const SYS_TICK: sys_tick::SysTick = sys_tick::SysTick;
pub const NVIC: *mut nvic::Nvic = 0xE000_E100 as *mut nvic::Nvic;
pub const SCB: *mut scb::Scb = 0xE000_ED00 as *mut scb::Scb;

pub unsafe extern "C" fn reset_handler() {
    crt0();
}
