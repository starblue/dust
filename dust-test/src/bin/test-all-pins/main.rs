// Tests all pins by outputting unique patterns
//
// HVQFN33 package
// Pin  LPC802   LPC804   LPC824   LPC845
//   1  PIO0_17  PIO0_17  PIO0_13  PIO0_13
//   2  PIO0_13  PIO0_13  PIO0_12  PIO0_12
//   3  PIO0_12  PIO0_12  nRESET   nRESET
//   4  nRESET   nRESET   PIO0_4   PIO0_4
//   5  PIO0_4   PIO0_4   PIO0_28  PIO0_28
//   6  SWCLK    SWCLK    SWCLK    SWCLK
//   7  SWDIO    SWDIO    SWDIO    SWDIO
//   8  PIO0_11  PIO0_11  PIO0_11  PIO0_11
//   9  PIO0_10  PIO0_10  PIO0_10  PIO0_10
//  10  n.c.     PIO0_21  PIO0_16  PIO0_16
//  11  n.c.     PIO0_29  PIO0_27  PIO0_27
//  12  n.c.     PIO0_28  PIO0_26  PIO0_26
//  13  n.c.     PIO0_27  PIO0_25  PIO0_25
//  14  n.c.     PIO0_26  PIO0_24  PIO0_24
//  15  n.c.     PIO0_20  PIO0_15  PIO0_15
//  16  PIO0_15  PIO0_15  PIO0_1   PIO0_1
//  17  PIO0_1   PIO0_1   PIO0_9   PIO0_9
//  18  PIO0_9   PIO0_9   PIO0_8   PIO0_8
//  19  PIO0_8   PIO0_8   VDD      VDD
//  20  VDD      VDD      VREFN    VREFN
//  21  n.c.     PIO0_30  VREFP    VREFP
//  22  PIO0_7   PIO0_7   PIO0_7   PIO0_7
//  23  VREFP    VREFP    PIO0_6   PIO0_6
//  24  PIO0_0   PIO0_0   PIO0_0   PIO0_0
//  25  PIO0_14  PIO0_14  PIO0_14  PIO0_14
//  26  n.c.     PIO0_19  PIO0_23  PIO0_23
//  27  n.c.     PIO0_25  PIO0_22  PIO0_22
//  28  n.c.     PIO0_24  PIO0_21  PIO0_21
//  29  n.c.     PIO0_23  PIO0_20  PIO0_20
//  30  n.c.     PIO0_22  PIO0_19  PIO0_19
//  31  n.c.     PIO0_18  PIO0_18  PIO0_18
//  32  PIO0_16  PIO0_16  PIO0_17  PIO0_17
//  33  VSS      VSS      VSS      VSS

#![feature(asm)]
#![no_std]
#![no_main]

extern crate dust;
#[cfg(feature = "lpc8xx")]
extern crate dust_lpc8xx;

use core::cmp::min;

use dust::gpio::port::{Clr, Set};
#[cfg(all(feature = "lpc8xx", not(feature = "lpc81x")))]
use dust::gpio::port::{DirClr, DirSet};

#[cfg(feature = "lpc8xx")]
use dust_lpc8xx::gpio;
#[cfg(feature = "lpc8xx")]
use dust_lpc8xx::GPIO;

fn delay(n: usize) {
    for _ in 0..n {
        // Make sure the loop is not optimized away
        unsafe { asm!("" :::: "volatile") }
    }
}

#[cfg(feature = "lpc802")]
const PIN_MASK: u32 = 0x0003ff93;
#[cfg(feature = "lpc804")]
const PIN_MASK: u32 = 0x7fffff93;
#[cfg(feature = "lpc82x")]
const PIN_MASK: u32 = 0x1fffffd3;
#[cfg(feature = "lpc845")]
// TODO disable CAPT_YL on PIO0_8 and CAPT_YH on PIO0_9
const PIN_MASK: u32 = 0x1ffffcd3;
//const PIN_MASK: u32 = 0x00000001;

#[cfg(feature = "lpc802")]
const DELAY: usize = 6000;
#[cfg(feature = "lpc804")]
const DELAY: usize = 7000;
#[cfg(feature = "lpc82x")]
const DELAY: usize = 8480;
#[cfg(feature = "lpc845")]
const DELAY: usize = 7630;

#[cfg(any(feature = "lpc802", feature = "lpc804"))]
fn enable_gpio_clock() {
    use dust_lpc8xx::syscon;
    use dust_lpc8xx::SYSCON;
    unsafe {
        let syscon = &mut *SYSCON;
        syscon.enable_clock(syscon::CLOCK_GPIO0);
    }
}

#[cfg(feature = "lpc84x")]
fn enable_gpio_clock() {
    use dust_lpc8xx::syscon;
    use dust_lpc8xx::SYSCON;
    unsafe {
        let syscon = &mut *SYSCON;
        syscon.enable_clock(syscon::CLOCK_GPIO0);
        syscon.enable_clock(syscon::CLOCK_GPIO1);
    }
}

#[cfg(not(any(feature = "lpc802", feature = "lpc804", feature = "lpc84x")))]
fn enable_gpio_clock() {
    // GPIO clock is already enabled after reset
}

#[cfg(feature = "lpc8xx")]
fn get_gpio_port() -> gpio::Port<'static> {
    let gpio = unsafe { &mut *GPIO };
    let port_index = 0;
    let port = unsafe { gpio::Port::new(gpio, port_index) };
    port
}

#[no_mangle]
pub fn main() {
    enable_gpio_clock();
    let mut gpio_port = get_gpio_port();
    let d = DELAY;
    loop {
        for pin in 0..min(gpio::PINS, 32) {
            if (1 << pin) & PIN_MASK != 0 {
                // set pin to output
                gpio_port.dir_set(1 << pin);

                // start bit
                gpio_port.clr_bits(1 << pin);
                delay(d);

                for bit in 0..8 {
                    if pin & (1 << bit) != 0 {
                        gpio_port.set_bits(1 << pin);
                    } else {
                        gpio_port.clr_bits(1 << pin);
                    }
                    delay(d);
                }
                // stop bit
                gpio_port.set_bits(1 << pin);
                delay(d);

                // set pin to input (high impedance)
                gpio_port.dir_clr(1 << pin);
            }
        }
    }
}
