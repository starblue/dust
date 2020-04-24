#![feature(llvm_asm)]
#![no_std]
#![no_main]

use core::fmt;
use core::fmt::Write;

#[cfg(all(feature = "lpc8xx", not(feature = "lpc81x")))]
use dust::gpio::port::DirSet;
#[cfg(feature = "lpc81x")]
use dust::gpio::port::DirSetValue;
use dust::gpio::port::{Clr, Set};

#[cfg(feature = "lpc8xx")]
use dust_lpc8xx::gpio;
#[cfg(feature = "lpc8xx")]
use dust_lpc8xx::GPIO;

use dust_lpc8xx::swm::{U0_RXD, U0_TXD};
#[cfg(any(feature = "lpc802", feature = "lpc804", feature = "lpc84x"))]
use dust_lpc8xx::syscon;
use dust_lpc8xx::syscon::Syscon;
use dust_lpc8xx::syscon::CLOCK_SWM;
use dust_lpc8xx::syscon::CLOCK_UART0;
#[cfg(any(feature = "lpc802", feature = "lpc804", feature = "lpc84x"))]
use dust_lpc8xx::syscon::RESET_UART0;
#[cfg(any(feature = "lpc81x", feature = "lpc82x", feature = "lpc83x"))]
use dust_lpc8xx::syscon::RESET_USART0;
use dust_lpc8xx::SWM;
use dust_lpc8xx::SYSCON;
use dust_lpc8xx::USART;

fn delay(n: usize) {
    for _ in 0..n {
        // Make sure the loop is not optimized away
        unsafe { llvm_asm!("" :::: "volatile") }
    }
}

#[cfg(any(feature = "lpc802", feature = "lpc804"))]
const LED: (usize, usize) = (0, 15);
#[cfg(feature = "lpc810")]
const LED: (usize, usize) = (0, 1);
#[cfg(feature = "lpc812")]
const LED: (usize, usize) = (0, 15);
#[cfg(any(feature = "lpc82x", feature = "lpc83x", feature = "lpc84x"))]
const LED: (usize, usize) = (0, 0);

#[cfg(any(feature = "lpc802", feature = "lpc804"))]
fn enable_gpio_clock() {
    unsafe {
        let syscon = &mut *SYSCON;
        syscon.enable_clock(syscon::CLOCK_GPIO0);
    }
}

#[cfg(feature = "lpc84x")]
fn enable_gpio_clock() {
    unsafe {
        let syscon = &mut *SYSCON;
        syscon.enable_clock(syscon::CLOCK_GPIO0);
        syscon.enable_clock(syscon::CLOCK_GPIO1);
    }
}

#[cfg(not(any(feature = "lpc802", feature = "lpc804", feature = "lpc84x",)))]
fn enable_gpio_clock() {
    // GPIO clock is already enabled after reset
}

#[cfg(feature = "lpc81x")]
fn init_gpio_port(port: &mut gpio::Port, bit_index: usize) {
    port.dir_modify_value(|w| w | (1 << bit_index));
}

#[cfg(all(feature = "lpc8xx", not(feature = "lpc81x")))]
fn init_gpio_port(port: &mut gpio::Port, bit_index: usize) {
    port.dir_set(1 << bit_index);
}

#[cfg(feature = "lpc8xx")]
fn get_gpio_port() -> gpio::Port<'static> {
    let gpio = unsafe { &mut *GPIO };
    let (port_index, bit_index) = LED;
    let mut port = unsafe { gpio::Port::new(gpio, port_index) };
    init_gpio_port(&mut port, bit_index);
    port
}

#[cfg(feature = "lpc8xx")]
struct Uart<'a>(&'a dust_lpc8xx::usart::Usart);

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

fn init_uart() -> Uart<'static> {
    let syscon = unsafe { &mut *SYSCON };
    let usart = &USART[0];

    // Configure SYSCON for USART0
    init_uart0_syscon(syscon);

    // initialize UART
    usart.init(UART_BRG_DIVISOR);
    Uart(usart)
}

impl<'a> fmt::Write for Uart<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            self.0.tx(b);
        }
        Ok(())
    }
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

    enable_gpio_clock();
    let mut gpio_port = get_gpio_port();

    let mut uart = init_uart();
    let d = 1000000;
    let mut n = 0;
    loop {
        write!(uart, "Hello {}! - ", n).ok();
        n += 1;

        let (_, bit_index) = LED;
        gpio_port.clr_bits(1 << bit_index);
        write!(uart, "blink high - ").ok();
        delay(d);

        gpio_port.set_bits(1 << bit_index);
        writeln!(uart, "blink low").ok();
        delay(d);
    }
}
