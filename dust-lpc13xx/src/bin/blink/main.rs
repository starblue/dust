#![feature(asm)]
#![feature(const_fn)]
#![feature(lang_items)]
#![feature(linkage)]
#![feature(naked_functions)]
#![feature(used)]
#![no_std]
#![no_main]

extern crate dust_lpc13xx;

use dust_lpc13xx::GPIO3;


fn delay(n: usize) {
    for _ in 0 .. n {
        // Make sure the loop is not optimized away
        unsafe { asm!("" :::: "volatile") }
    }
}


const LED: usize = 0;

#[no_mangle]
pub fn main() {
    let gpio = unsafe { &mut *GPIO3 };

    gpio.set_pin_output(LED);
    let d = 700000;
    let n = 2;
    loop {
        for _ in 0 .. n {
            gpio.set_pin(LED);
            delay(d);
            gpio.clr_pin(LED);
            delay(d);
        }
        for _ in 0 .. n / 2 {
            gpio.set_pin(LED);
            delay(2 * d);
            gpio.clr_pin(LED);
            delay(2 * d);
        }
    }
}
