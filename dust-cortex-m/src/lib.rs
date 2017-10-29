#![feature(asm)]
#![no_std]

extern crate volatile_register;

pub mod intrinsics;

pub mod sys_tick;
pub mod nvic;
pub mod scb;


pub const SYS_TICK: *mut sys_tick::SysTick = 0xE000_E010 as *mut sys_tick::SysTick;
pub const NVIC: *mut nvic::Nvic = 0xE000_E100 as *mut nvic::Nvic;
pub const SCB: *mut scb::Scb = 0xE000_ED00 as *mut scb::Scb;
