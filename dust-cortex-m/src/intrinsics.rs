//! Intrinsics for special Cortex-M instructions.
//!
//! We consider instructions special, if their effect is not available
//! through normal use of the compiler or standard library.
//!
//! The order follows the ARM v7-M Architecture Reference Manual (DDI403D).
//!
//! # A4.4.8 Miscellaneous data-processing instructions

#[cfg(feature = "v7m")]
#[inline(always)]
/// Reverse the bit order of a 32-bit value
pub fn rbit(a: u32) -> u32 {
    let result: u32;
    unsafe {
        asm!("rbit $0, $1"
             : "=r"(result)
             : "r"(a)
             :
             : );
    }
    result
}

// # A4.6.3 Unprivileged loads and stores
// TODO

// # A4.6.4 Exclusive loads and stores
// TODO

// LDREX
// LDREXB
// LDREXH
// STREX
// STREXB
// STREXH

// Clear Exclusive
// CLREX
// TODO

// Debug hint
// DBG

/// Data Memory Barrier (DMB) instruction
#[inline(always)]
pub fn dmb() {
    unsafe {
        asm!("dmb"
             :
             :
             : "memory"
             : "volatile");
    }
}

/// Data Synchronization Barrier (DSB) instruction
#[inline(always)]
pub fn dsb() {
    unsafe {
        asm!("dsb"
             :
             :
             : "memory"
             : "volatile");
    }
}

/// Instruction Synchronization Barrier (ISB) instruction
#[inline(always)]
pub fn isb() {
    unsafe {
        asm!("isb"
             :
             :
             : "memory"
             : "volatile");
    }
}

/// No Operation (NOP) instruction
#[inline(always)]
pub fn nop() {
    unsafe {
        asm!("nop"
             :
             :
             :
             : "volatile");
    }
}

// Preload Data
// PLD (immediate)
// PLD (literal)
// PLD (register)

// Preload Instruction
// PLI (immediate, literal)
// PLI (register)

/// Send Event (SEV) instruction
#[inline(always)]
pub fn sev() {
    unsafe {
        asm!("sev"
             :
             :
             : "memory"
             : "volatile");
    }
}

// Supervisor Call (SVC) instruction
// TODO unclear if useful out of context

/// Wait for Event (WFE) instruction
#[inline(always)]
pub fn wfe() {
    unsafe {
        asm!("wfe"
             :
             :
             : "memory"
             : "volatile");
    }
}

/// Wait for Interrupt (WFI) instruction
#[inline(always)]
pub fn wfi() {
    unsafe {
        asm!("wfi"
             :
             :
             : "memory"
             : "volatile");
    }
}

// Yield (YIELD) instruction
// TODO (collides with yield keyword)
//#[inline(always)]
// pub fn yield() {
//     unsafe {
//         asm!("yield"
//              :
//              :
//              :
//              : "volatile");
//     }
// }

/// # A4.9 Exception-generating instructions

/// Breakpoint (BKPT) instruction
#[inline(always)]
pub fn bkpt() {
    unsafe {
        asm!("bkpt"
             :
             :
             :
             : "volatile");
    }
}

/// Permanently Undefined (UDF) instruction
#[inline(always)]
pub fn udf() {
    unsafe {
        asm!("udf"
             :
             :
             :
             : "volatile");
    }
}

/// # Other instructions not in chapter A4

/// Change Processor State, interrupt enable (CPSIE I) instruction
#[inline(always)]
pub fn cpsie_i() {
    unsafe {
        asm!("cpsie i"
             :
             :
             : "memory"
             : "volatile");
    }
}

/// Change Processor State, interrupt disable (CPSID I) instruction
#[inline(always)]
pub fn cpsid_i() {
    unsafe {
        asm!("cpsid i"
             :
             :
             : "memory"
             : "volatile");
    }
}

/// Change Processor State, fault enable (CPSIE F) instruction
#[cfg(feature = "v7m")]
#[inline(always)]
pub fn cpsie_f() {
    unsafe {
        asm!("cpsie f"
             :
             :
             : "memory"
             : "volatile");
    }
}

/// Change Processor State, fault disable (CPSID F) instruction
#[cfg(feature = "v7m")]
#[inline(always)]
pub fn cpsid_f() {
    unsafe {
        asm!("cpsid f"
             :
             :
             : "memory"
             : "volatile");
    }
}
