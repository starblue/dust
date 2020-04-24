#![feature(llvm_asm)]
#![no_std]
#![no_main]

use core::ptr::read_volatile;

use dust_register::Write;

use dust_cortex_m::sys_tick;
use dust_cortex_m::SYS_TICK;

#[cfg(any(
    feature = "atsamd09",
    feature = "atsaml11",
    all(feature = "lpc8xx", not(feature = "lpc81x"))
))]
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

#[cfg(feature = "stm32f0")]
use dust_stm32f0::gpio;
#[cfg(feature = "stm32f0")]
use dust_stm32f0::GPIO;

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
#[cfg(feature = "lpc82x")]
const LED: (usize, usize) = (0, 15);
#[cfg(feature = "lpc83x")]
const LED: (usize, usize) = (0, 15);
#[cfg(feature = "lpc84x")]
const LED: (usize, usize) = (0, 0);
#[cfg(feature = "lpc1114")]
const LED: (usize, usize) = (0, 9);
#[cfg(feature = "lpc1343")]
const LED: (usize, usize) = (3, 0);
#[cfg(feature = "stm32f0")]
const LED: (usize, usize) = (0, 0);

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

#[cfg(feature = "stm32f0")]
fn enable_gpio_clock() {
    use dust_stm32f0::rcc;
    use dust_stm32f0::RCC;
    unsafe {
        let rcc = &mut *RCC;
        // enable clock for GPIOA
        rcc.ahbenr.modify(|w| {
            w | rcc::AHB_IOPA | rcc::AHB_IOPB | rcc::AHB_IOPC | rcc::AHB_IOPD | rcc::AHB_IOPF
        });
    }
}

#[cfg(not(any(
    feature = "lpc802",
    feature = "lpc804",
    feature = "lpc84x",
    feature = "stm32f0"
)))]
fn enable_gpio_clock() {
    // GPIO clock is already enabled after reset
}

#[cfg(any(feature = "lpc81x", feature = "lpc11xx", feature = "lpc13xx"))]
fn init_gpio_port(port: &mut gpio::Port, bit_index: usize) {
    port.dir_modify_value(|w| w | (1 << bit_index));
}

#[cfg(any(
    feature = "atsamd09",
    feature = "atsaml11",
    all(feature = "lpc8xx", not(feature = "lpc81x"))
))]
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

#[cfg(any(feature = "stm32f0"))]
fn get_gpio_port() -> gpio::Port<'static> {
    let (port_index, bit_index) = LED;
    let gpio = unsafe { &mut *GPIO[port_index].unwrap() };
    gpio.set_pin_output(bit_index);
    let port = unsafe { gpio::Port::new(gpio) };
    port
}

#[cfg(feature = "atsamd09")]
const CLOCK_FREQUENCY: u32 = 1_000_000;
#[cfg(feature = "atsaml11")]
const CLOCK_FREQUENCY: u32 = 4_000_000;
#[cfg(feature = "lpc8xx")]
const CLOCK_FREQUENCY: u32 = 12_000_000;
#[cfg(feature = "lpc11xx")]
const CLOCK_FREQUENCY: u32 = 12_000_000;
#[cfg(feature = "lpc13xx")]
const CLOCK_FREQUENCY: u32 = 12_000_000;
#[cfg(feature = "stm32f0x0")]
const CLOCK_FREQUENCY: u32 = 8_000_000;

/// Time since system startup in milliseconds
static mut TIME_MS: u32 = 0;

#[no_mangle]
pub unsafe extern "C" fn systick_handler() {
    TIME_MS += 1;
}

fn get_time_ms() -> u32 {
    unsafe { read_volatile(&TIME_MS) }
}

#[no_mangle]
pub fn main() {
    // set up systick interrupt
    unsafe {
        let sys_tick = SYS_TICK;

        // set reload value for one tick per millisecond
        sys_tick.rvr().write(CLOCK_FREQUENCY / 1000);

        // reset timer value
        sys_tick.cvr().write(0);

        // enable timer and timer interrupt
        sys_tick.csr().write(
            sys_tick::CSR_ENABLE  // enable timer
                   | sys_tick::CSR_TICKINT  // enable timer interrupt
                   | sys_tick::CSR_CLKSOURCE, // use core clock
        );

        // interrupt is already enabled after reset
    }

    enable_gpio_clock();
    let mut gpio_port = get_gpio_port();
    let delay_ms = 500;
    let (_, bit_index) = LED;

    let mut next_time = get_time_ms();
    loop {
        next_time += delay_ms;
        gpio_port.set_bits(1 << bit_index);
        while get_time_ms() < next_time {}

        next_time += delay_ms;
        gpio_port.clr_bits(1 << bit_index);
        while get_time_ms() < next_time {}
    }
}
