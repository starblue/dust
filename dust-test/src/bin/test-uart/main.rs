#![feature(asm)]
#![no_std]
#![no_main]

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

#[cfg(any(feature = "lpc802m001", feature = "lpc804m101", feature = "lpc81x", feature = "lpc82x", feature = "lpc83x"))]
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

fn init_uart() -> &'static mut Usart {
    let syscon = unsafe { &mut *SYSCON };
    let usart = unsafe { &mut *USART[0] };

    // Configure SYSCON for USART0
    init_uart0_syscon(syscon);

    // initialize UART
    usart.init(UART_BRG_DIVISOR);
    usart
}

#[no_mangle]
pub fn main() {
    // Configure switch matrix (SWM)
    let syscon = unsafe { &mut *SYSCON };
    let swm = unsafe { &mut *SWM };
    let (txd, rxd) = UART_PINS;
    unsafe {
        syscon.enable_clock(CLOCK_SWM);
        swm.set_movable_function_pin(U0_RXD, rxd);
        swm.set_movable_function_pin(U0_TXD, txd);
        syscon.disable_clock(CLOCK_SWM);
    }

    let uart = init_uart();
    let d = 1000000;
    loop {
        for c in b"Hello world!\r\n" {
            uart.tx(*c);
        }
        delay(d);
    }
}
