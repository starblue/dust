#![feature(const_fn)]
#![feature(linkage)]
#![no_std]

extern crate dust_cortex_m;

pub mod atsaml11_vectors;

pub mod port;

pub const PORT: *mut port::Regs = 0x40003000 as *mut port::Regs;
