#![feature(const_fn)]
#![feature(linkage)]
#![no_std]

extern crate dust_cortex_m;

pub mod atsamd09_vectors;

pub mod port;

pub const PORT: *mut port::Regs = 0x41004400 as *mut port::Regs;
