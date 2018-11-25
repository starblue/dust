#![feature(asm)]
#![no_std]
#![no_main]

use dust_lpc8xx::swm::{U0_RXD, U0_TXD};
use dust_lpc8xx::syscon::CLOCK_SWM;
use dust_lpc8xx::syscon::CLOCK_UART0;
use dust_lpc8xx::syscon::RESET_USART0;
use dust_lpc8xx::SWM;
use dust_lpc8xx::SYSCON;
use dust_lpc8xx::USART;

fn delay(n: usize) {
    for _ in 0..n {
        // Make sure the loop is not optimized away
        unsafe { asm!("" :::: "volatile") }
    }
}

#[no_mangle]
pub fn main() {
    let syscon = unsafe { &mut *SYSCON };
    let swm = unsafe { &mut *SWM };
    let usart = unsafe { &mut *USART[0] };

    // Configure switch matrix (SWM)
    unsafe {
        syscon.enable_clock(CLOCK_SWM);
        swm.set_movable_function_pin(U0_RXD, 0);
        swm.set_movable_function_pin(U0_TXD, 4);
        syscon.disable_clock(CLOCK_SWM);
    }

    // Configure SYSCON for USART0
    unsafe {
        syscon.enable_clock(CLOCK_UART0);
        syscon.periph_reset(RESET_USART0);

        // Generate suitable clock for serial port
        // U_PCLK = main_clock / (1 + MULT / DIV)
        // 12000000 / (1 + 160 / 256)
        // 12000000 / ((256 + 160) / 256)
        // 12000000 / (416 / 256)
        // 12000000 / (13 / 8)
        // 12000000 * 8 / 13
        // 96000000 / 13
        //  7384615 ~(0.16%) 7372800 = 4 * 16 * 115200
        syscon.uartclkdiv.write(1);
        syscon.uartfrgdiv.write(0xff);
        syscon.uartfrgmult.write(0xa0);
    }

    usart.init();

    let d = 1000000;
    loop {
        for c in b"Hello world!\r\n" {
            usart.tx(*c);
        }
        delay(d);
    }
}
