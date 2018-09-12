#![feature(asm)]
#![no_std]
#![no_main]

#[cfg(any(feature = "atsamd09", feature = "atsaml11", all(feature = "lpc8xx", not(feature = "lpc81x"))))]
use dust::gpio::port::DirSet;
#[cfg(any(feature = "lpc81x", feature = "lpc11xx", feature = "lpc13xx"))]
use dust::gpio::port::DirSetValue;
use dust::gpio::port::{Clr, Set};

#[cfg(feature = "atsamd09")]
use dust_atsamd09::port as gpio;
#[cfg(feature = "atsamd09")]
use dust_atsamd09::PORT as GPIO;

#[cfg(feature = "atsaml11")]
use dust_atsaml11::port as gpio;
#[cfg(feature = "atsaml11")]
use dust_atsaml11::PORT as GPIO;

#[cfg(feature = "lpc8xx")]
use dust_lpc8xx::gpio;
#[cfg(feature = "lpc8xx")]
use dust_lpc8xx::GPIO;

#[cfg(feature = "lpc11xx")]
use dust_lpc11xx::gpio;
#[cfg(feature = "lpc11xx")]
use dust_lpc11xx::GPIO;

#[cfg(feature = "lpc13xx")]
use dust_lpc13xx::gpio;
#[cfg(feature = "lpc13xx")]
use dust_lpc13xx::GPIO;

fn delay(n: usize) {
    for _ in 0..n {
        // Make sure the loop is not optimized away
        unsafe { asm!("" :::: "volatile") }
    }
}

#[cfg(feature = "atsamd09")]
const LED: (usize, usize) = (0, 24);
#[cfg(feature = "atsaml11")]
const LED: (usize, usize) = (0, 0);
#[cfg(any(feature = "lpc802", feature = "lpc804"))]
const LED: (usize, usize) = (0, 15);
#[cfg(feature = "lpc810")]
const LED: (usize, usize) = (0, 1);
#[cfg(feature = "lpc812")]
const LED: (usize, usize) = (0, 15);
#[cfg(any(feature = "lpc82x", feature = "lpc84x"))]
const LED: (usize, usize) = (0, 0);
#[cfg(feature = "lpc1114")]
const LED: (usize, usize) = (0, 9);
#[cfg(feature = "lpc1343")]
const LED: (usize, usize) = (3, 0);

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

#[cfg(any(feature = "lpc81x", feature = "lpc11xx", feature = "lpc13xx"))]
fn init_gpio_port(port: &mut gpio::Port, bit_index: usize) {
    port.dir_modify_value(|w| w | (1 << bit_index));
}

#[cfg(any(feature = "atsamd09", feature = "atsaml11", all(feature = "lpc8xx", not(feature = "lpc81x"))))]
fn init_gpio_port(port: &mut gpio::Port, bit_index: usize) {
    port.dir_set(1 << bit_index);
}

#[cfg(any(feature = "atsamd09", feature = "atsaml11"))]
fn get_gpio_port() -> gpio::Port<'static> {
    let gpio = unsafe { &mut *GPIO };
    let (_port_index, bit_index) = LED;
    let mut port = unsafe { gpio::Port::new(gpio) };
    init_gpio_port(&mut port, bit_index);
    port
}

#[cfg(feature = "lpc8xx")]
fn get_gpio_port() -> gpio::Port<'static> {
    let gpio = unsafe { &mut *GPIO };
    let (port_index, bit_index) = LED;
    let mut port = unsafe { gpio::Port::new(gpio, port_index) };
    init_gpio_port(&mut port, bit_index);
    port
}

#[cfg(any(feature = "lpc11xx", feature = "lpc13xx"))]
fn get_gpio_port() -> gpio::Port<'static> {
    let (port_index, bit_index) = LED;
    let gpio = unsafe { &mut *GPIO[port_index] };
    let mut port = unsafe { gpio::Port::new(gpio) };
    init_gpio_port(&mut port, bit_index);
    port
}

#[no_mangle]
pub fn main() {
    enable_gpio_clock();
    let mut gpio_port = get_gpio_port();
    let d = 100000;
    let n = 2;
    loop {
        for _ in 0..n {
            let (_, bit_index) = LED;
            gpio_port.set_bits(1 << bit_index);
            delay(2 * d);
            gpio_port.clr_bits(1 << bit_index);
            delay(d);
        }
    }
}
