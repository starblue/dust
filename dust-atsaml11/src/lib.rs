#![feature(const_fn)]
#![feature(linkage)]
#![no_std]

#[macro_use]
extern crate dust;
extern crate dust_cortex_m;
extern crate volatile_register;

pub mod atsaml11_vectors;

pub mod port;

pub const PORT: *mut port::Regs = 0x40003000 as *mut port::Regs;
