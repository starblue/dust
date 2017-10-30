#![feature(asm)]
#![no_std]
#![no_main]

#[cfg(feature = "lpc8xx")]
extern crate dust_lpc8xx;

#[cfg(feature = "lpc11xx")]
extern crate dust_lpc11xx;

use dust_lpc8xx::GPIO;
use dust_lpc8xx::SWM;
use dust_lpc8xx::SYSCON;
use dust_lpc8xx::USART0;
use dust_lpc8xx::swm::{U0_RXD, U0_TXD};
use dust_lpc8xx::syscon::CLOCK_SWM;
use dust_lpc8xx::syscon::CLOCK_UART0;
use dust_lpc8xx::syscon::RESET_USART0;


fn delay(n: usize) {
    for _ in 0 .. n {
        // Make sure the loop is not optimized away
        unsafe { asm!("" :::: "volatile") }
    }
}


#[cfg(feature = "lpc810")]
const LED: usize = 1;
#[cfg(any(feature = "lpc812", feature = "lpc824"))]
const LED: usize = 15;
#[cfg(feature = "lpc845")]
const LED: usize = 0;
#[cfg(feature = "lpc1114")]
const LED: usize = 9;

#[no_mangle]
pub fn main() {
    let syscon = unsafe { &mut *SYSCON };
    let swm = unsafe { &mut *SWM };
    let gpio = unsafe { &mut *GPIO };
    let usart = unsafe { &mut *USART0 };

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

    gpio.set_pin_output(LED);
    let d = 500000;
    let n = 2;
    loop {
        for c in b"Hello world!\r\n" {
            usart.tx(*c);
        }
        for _ in 0 .. n {
            gpio.set_pin(LED);
            delay(d);
            gpio.clr_pin(LED);
            delay(d);
        }
    }
}
