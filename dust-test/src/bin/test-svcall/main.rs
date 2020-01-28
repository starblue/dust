#![feature(asm)]
#![feature(naked_functions)]
#![no_std]
#![no_main]

use dust_cortex_m::scb::SCB_CCR_STKALIGN;
use dust_cortex_m::EXC_RETURN_USE_PSP;
use dust_cortex_m::SCB;

// This dependency is needed to pull in the hard-fault handler
#[allow(unused)]
use dust_hardfault::HARD_FAULT_HANDLER;

use dust_lpc8xx::swm::{U0_RXD, U0_TXD};
use dust_lpc8xx::syscon::Syscon;
use dust_lpc8xx::syscon::CLOCK_SWM;
use dust_lpc8xx::syscon::CLOCK_UART0;
#[cfg(any(feature = "lpc802", feature = "lpc804", feature = "lpc84x"))]
use dust_lpc8xx::syscon::RESET_UART0;
#[cfg(any(feature = "lpc81x", feature = "lpc82x", feature = "lpc83x"))]
use dust_lpc8xx::syscon::RESET_USART0;
use dust_lpc8xx::usart::Usart;
use dust_lpc8xx::SWM;
use dust_lpc8xx::SYSCON;
use dust_lpc8xx::USART;

fn delay(n: usize) {
    for _ in 0..n {
        // Make sure the loop is not optimized away
        unsafe { asm!("" :::: "volatile") }
    }
}

#[cfg(any(
    feature = "lpc802m001",
    feature = "lpc804m101",
    feature = "lpc81x",
    feature = "lpc82x",
    feature = "lpc83x"
))]
const UART_PINS: (usize, usize) = (4, 0);
#[cfg(any(feature = "lpc802m011", feature = "lpc804m111"))]
const UART_PINS: (usize, usize) = (9, 8);
#[cfg(feature = "lpc84x")]
const UART_PINS: (usize, usize) = (25, 24);

/// Multiplier setting for fractional baud rate generators
///
/// This allows up to 460800 baud from the default 12 MHz IRC/FRO clock.
/// A divisor of 4 in the UART baud rate generator
/// then gives the standard baud rate of 115200 baud.
/// The factor of 16 results from the default number of samples per bit.
///
/// U_PCLK = main_clock / (1 + MULT / DIV)
/// 12000000 / (1 + 160 / 256)
/// 12000000 / ((256 + 160) / 256)
/// 12000000 / (416 / 256)
/// 12000000 / (13 / 8)
/// 12000000 * 8 / 13
/// 96000000 / 13
///  7384615 ~(0.16%) 7372800 = 4 * 16 * 115200
///
/// TODO Use 13 samples per bit and avoid the FRG?
///
const FRG_MULT: u32 = 160;

/// Divider setting for fractional baud rate generators
///
/// Must be set to 0xff, the only supported value.
const FRG_DIV: u32 = 0xff;

/// Divisor for the baud rate generator
const UART_BRG_DIVISOR: u32 = 4;

#[cfg(any(feature = "lpc81x", feature = "lpc82x", feature = "lpc83x"))]
fn init_uart0_syscon(syscon: &mut Syscon) {
    unsafe {
        syscon.enable_clock(CLOCK_UART0);
        syscon.periph_reset(RESET_USART0);

        // Divide the input clock by 1
        syscon.uartclkdiv.write(1);

        syscon.uartfrgdiv.write(FRG_DIV);
        syscon.uartfrgmult.write(FRG_MULT);
    }
}
#[cfg(any(feature = "lpc802", feature = "lpc804", feature = "lpc84x"))]
fn init_uart0_syscon(syscon: &mut Syscon) {
    unsafe {
        syscon.enable_clock(CLOCK_UART0);
        syscon.periph_reset(RESET_UART0);

        syscon.frg0div.write(FRG_DIV);
        syscon.frg0mult.write(FRG_MULT);

        // Select main clock as input clock for FRG0
        syscon.frg0clksel.write(1);

        // Select FRG0 as clock for USART0
        syscon.uart0clksel.write(2);
    }
}

fn init_uart() -> &'static Usart {
    let syscon = unsafe { &mut *SYSCON };
    let usart = &USART[0];

    // Configure SYSCON for USART0
    init_uart0_syscon(syscon);

    // initialize UART
    usart.init(UART_BRG_DIVISOR);
    usart
}

#[no_mangle]
extern "C" fn hard_fault_write_byte(b: u8) {
    let uart = &USART[0];
    uart.tx(b);
}

/// SVCall handler.
///
/// This implementation uses an assembly wrapper to save lr, msp and psp
/// and then jumps to svcall_handler_rust to do the main work.
#[no_mangle]
#[naked]
pub unsafe extern "C" fn svcall_handler() {
    asm!("mov r0, lr
          mrs r1, msp
          mrs r2, psp
          b svcall_handler_rust"
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

#[no_mangle]
unsafe extern "C" fn svcall_handler_rust(lr: u32, msp: u32, psp: u32) {
    let sp = if (lr & EXC_RETURN_USE_PSP) == 0 {
        msp
    } else {
        psp
    };
    let stack_frame = &mut *(sp as *mut StackFrame);
    let pc = stack_frame.pc;
    // immediate value of svc instruction
    let n = *((pc - 2) as *const u8);
    // TODO implement svc handler
    write_nl();
    write_bytes(b"SVCall");
    write_nl();
    write_reg(b"n: ", u32::from(n));
    write_reg(b"r0: ", stack_frame.r0);
    write_reg(b"r1: ", stack_frame.r1);
    write_reg(b"r2: ", stack_frame.r2);
    write_reg(b"r3: ", stack_frame.r3);

    stack_frame.r0 = stack_frame.r0 + stack_frame.r1 + stack_frame.r2 + stack_frame.r3;
}

fn write_byte(data: u8) {
    hard_fault_write_byte(data);
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

#[no_mangle]
pub fn main() {
    unsafe {
        // set stack alignment on exception entry to 8 bytes
        let scb = &mut *SCB;
        scb.ccr.modify(|w| w | SCB_CCR_STKALIGN);

        // set exception priorities

        // PENDSV has lowest priority
        let pendsv_prio = 0xc0;
        // SVC and SYSTICK have second lowest priority
        let svcall_prio = 0x80;
        let systick_prio = 0x80;
        // set SHPR2
        scb.shpr[1].modify(|w| (w & 0x00ffffff) | (svcall_prio << 24));
        // set SHPR3
        scb.shpr[2].modify(|w| (w & 0x0000ffff) | (pendsv_prio << 16) | (systick_prio << 24));
    }

    // Configure switch matrix (SWM)
    let syscon = unsafe { &mut *SYSCON };
    let swm = unsafe { &mut *SWM };
    let (txd_pin, rxd_pin) = UART_PINS;
    unsafe {
        syscon.enable_clock(CLOCK_SWM);
        swm.set_movable_function_pin(U0_RXD, rxd_pin);
        swm.set_movable_function_pin(U0_TXD, txd_pin);
        syscon.disable_clock(CLOCK_SWM);
    }
    let uart = init_uart();

    let d = 1_000_000;
    let mut i = 0;
    loop {
        for c in b"Still alive!\r\n" {
            uart.tx(*c);
        }
        const N: u8 = 23;
        let in0 = i + 1;
        let in1 = i + 2;
        let in2 = i + 3;
        let in3 = i + 4;
        let out: u32;
        unsafe {
            asm!("svc ${1}"
                 : "={r0}"(out)
                 : "i"(N), "{r0}"(in0), "{r1}"(in1), "{r2}"(in2), "{r3}"(in3)
                 :
                 : "volatile");
        }
        write_reg(b"result: ", out);
        i += 1;

        delay(d);
    }
}
