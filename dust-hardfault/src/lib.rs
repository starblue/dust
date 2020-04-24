#![no_std]
#![feature(llvm_asm)]
#![feature(naked_functions)]

use dust_cortex_m::EXC_RETURN_USE_PSP;
use dust_cortex_m::SCB;

/// Write a byte to the output device, blocking as necessary.
///
/// Implement this function for your system.
extern "C" {
    fn hard_fault_write_byte(b: u8);
}

const STACK_DUMP_SIZE: u32 = 256;
const EMERGENCY_STACK_SIZE: usize = 256;

#[no_mangle]
// #[repr(align(8))] isn't allowed for arrays
// use u64 and hope it is aligned to 8 bytes
pub static mut EMERGENCY_STACK: [u64; EMERGENCY_STACK_SIZE / 8] = [0_u64; EMERGENCY_STACK_SIZE / 8];

/// Import this constant to create an artificial dependency.
///
/// This is needed to pull the hard-fault handler into the executable.
#[no_mangle]
pub static HARD_FAULT_HANDLER: unsafe extern "C" fn() = hard_fault_handler;

/// Hard-fault handler.
///
/// This implementation uses an assembly wrapper to save all relevant registers
/// and then jump to hard_fault_handler_rust to do the main work.
#[no_mangle]
#[naked]
pub unsafe extern "C" fn hard_fault_handler() {
    llvm_asm!(
        "mov r0, lr
         mrs r1, msp
         mrs r2, psp
         ldr r3, =EMERGENCY_STACK + 256
         mov sp, r3
         push {r4-r7}
         mov r4, r8
         mov r5, r9
         mov r6, r10
         mov r7, r11
         push {r4-r7}
         mov r3, sp
         b hard_fault_handler_rust"
        :
        :
        :
        : "volatile"
    )
}

#[repr(C)]
struct StackFrame {
    r0: u32,
    r1: u32,
    r2: u32,
    r3: u32,
    r12: u32,
    lr: u32,
    pc: u32,
    xpsr: u32,
}

#[repr(C)]
struct CalleeSavedRegs {
    r4: u32,
    r5: u32,
    r6: u32,
    r7: u32,
    r8: u32,
    r9: u32,
    r10: u32,
    r11: u32,
}

#[no_mangle]
unsafe extern "C" fn hard_fault_handler_rust(
    lr: u32,
    msp: u32,
    psp: u32,
    callee_saved_regs: &CalleeSavedRegs,
) -> ! {
    let sp = if (lr & EXC_RETURN_USE_PSP) == 0 {
        msp
    } else {
        psp
    };
    let stack_frame = &*(sp as *const StackFrame);

    let scb = &mut *SCB;
    let bfar = scb.bfar.read();
    let mmfar = scb.mmfar.read();
    let cfsr = scb.cfsr.read();

    write_nl();
    write_bytes(b"Hard fault");
    write_nl();
    write_nl();
    write_reg(b"EXC_RETURN: ", lr);
    write_reg(b"msp: ", msp);
    write_reg(b"psp: ", psp);
    write_nl();
    write_reg(b"cfsr:  ", cfsr);
    write_reg(b"bfar:  ", bfar);
    write_reg(b"mmfar: ", mmfar);
    write_nl();
    write_reg(b"xpsr: ", stack_frame.xpsr);
    write_nl();
    write_reg(b"r0:  ", stack_frame.r0);
    write_reg(b"r1:  ", stack_frame.r1);
    write_reg(b"r2:  ", stack_frame.r2);
    write_reg(b"r3:  ", stack_frame.r3);
    write_reg(b"r4:  ", callee_saved_regs.r4);
    write_reg(b"r5:  ", callee_saved_regs.r5);
    write_reg(b"r6:  ", callee_saved_regs.r6);
    write_reg(b"r7:  ", callee_saved_regs.r7);
    write_reg(b"r8:  ", callee_saved_regs.r8);
    write_reg(b"r9:  ", callee_saved_regs.r9);
    write_reg(b"r10: ", callee_saved_regs.r10);
    write_reg(b"r11: ", callee_saved_regs.r11);
    write_reg(b"r12: ", stack_frame.r12);
    write_reg(b"lr:  ", stack_frame.lr);
    write_reg(b"pc:  ", stack_frame.pc);
    write_nl();
    write_bytes(b"stack:");
    write_nl();
    // align address to 16 bytes
    let mut a = sp & 0xfffffff0;
    while a < sp + STACK_DUMP_SIZE {
        if a % 16 == 0 {
            write_32(a);
            write_byte(b' ');
        }

        if a >= sp {
            write_byte(b' ');
            let d = *(a as *const u32);
            write_32(d);
        } else {
            write_bytes(b"         ");
        }
        a += 4;

        if a % 16 == 0 {
            write_nl();
        }
    }

    loop {}
}

fn write_byte(data: u8) {
    unsafe {
        hard_fault_write_byte(data);
    }
}

fn write_bytes(data: &[u8]) {
    for b in data {
        write_byte(*b);
    }
}

fn write_nl() {
    write_bytes(b"\r\n");
}

const HEX_DIGITS: [u8; 16] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b', b'c', b'd', b'e', b'f',
];
/// Write a nibble.
fn write_4(data: u32) {
    let i = (data & 0xf) as usize;
    let c = HEX_DIGITS[i];
    write_byte(c);
}

/// Write a byte.
fn write_8(data: u32) {
    write_4((data >> 4) & 0xf);
    write_4((data >> 0) & 0xf);
}

/// Write a halfword.
fn write_16(data: u32) {
    write_8((data >> 8) & 0xff);
    write_8((data >> 0) & 0xff);
}

/// Write a word.
fn write_32(data: u32) {
    write_16((data >> 16) & 0xffff);
    write_16((data >> 0) & 0xffff);
}

/// Dump a register in a nice format
fn write_reg(header: &[u8], data: u32) {
    write_bytes(header);
    write_32(data);
    write_nl();
}
