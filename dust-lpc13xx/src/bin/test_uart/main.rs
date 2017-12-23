#![feature(asm)]
#![no_std]
#![no_main]

extern crate dust_lpc13xx;

use dust_lpc13xx::GPIO3;
use dust_lpc13xx::IOCON;
use dust_lpc13xx::SYSCON;
use dust_lpc13xx::UART;
use dust_lpc13xx::iocon::IOCON_MODE_PULL_UP;
use dust_lpc13xx::iocon::PIO1_6;
use dust_lpc13xx::iocon::PIO1_7;
use dust_lpc13xx::syscon::CLOCK_IOCON;
use dust_lpc13xx::syscon::CLOCK_UART;

fn delay(n: usize) {
    for _ in 0..n {
        // Make sure the loop is not optimized away
        unsafe { asm!("" :::: "volatile") }
    }
}

const LED: usize = 0;

#[no_mangle]
pub fn main() {
    let syscon = unsafe { &mut *SYSCON };
    let iocon = unsafe { &mut *IOCON };
    let gpio = unsafe { &mut *GPIO3 };
    let uart = unsafe { &mut *UART };

    // Configure pin functions for UART
    unsafe {
        syscon.enable_clock(CLOCK_IOCON);
        // Select RXD and TXD functions
        iocon.write(PIO1_6, 1 | IOCON_MODE_PULL_UP);
        iocon.write(PIO1_7, 1 | IOCON_MODE_PULL_UP);
    }

    // Configure SYSCON for UART
    unsafe {
        syscon.enable_clock(CLOCK_UART);
        syscon.uartclkdiv.write(1);
    }

    uart.init();

    gpio.set_pin_output(LED);
    let d = 500000;
    let n = 2;
    loop {
        for c in b"Hello world!\r\n" {
            uart.tx(*c);
        }
        for _ in 0..n {
            gpio.set_pin(LED);
            delay(d);
            gpio.clr_pin(LED);
            delay(d);
        }
        for _ in 0..n / 2 {
            gpio.set_pin(LED);
            delay(2 * d);
            gpio.clr_pin(LED);
            delay(2 * d);
        }
    }
}
