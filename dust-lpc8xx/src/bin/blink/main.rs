#![feature(asm)]
#![feature(const_fn)]
#![feature(lang_items)]
#![feature(linkage)]
#![feature(naked_functions)]
#![feature(used)]
#![no_std]
#![no_main]

extern crate dust_lpc8xx;

use dust_lpc8xx::GPIO;

fn delay(n: usize) {
    for _ in 0..n {
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

#[no_mangle]
pub fn main() {
    let gpio = unsafe { &mut *GPIO };

    gpio.set_pin_output(LED);
    let d = 500000;
    let n = 2;
    loop {
        for _ in 0..n {
            gpio.set_pin(LED);
            delay(d);
            gpio.clr_pin(LED);
            delay(d);
        }
        for _ in 0..n {
            gpio.toggle_pin(LED);
            delay(2 * d);
        }
    }
}
