#![feature(asm)]
#![feature(const_fn)]
#![feature(lang_items)]
#![feature(linkage)]
#![feature(naked_functions)]
#![feature(used)]
#![no_std]
#![no_main]

extern crate dust;
#[cfg(feature = "lpc8xx")]
extern crate dust_lpc8xx;

use dust::gpio::port::DirSet;
use dust::gpio::port::{Clr, Set};

use dust_lpc8xx::gpio;
use dust_lpc8xx::GPIO;

use dust_lpc8xx::syscon;
use dust_lpc8xx::SYSCON;

use dust_lpc8xx::swm;
use dust_lpc8xx::SWM;

use dust_lpc8xx::plu::LutInputMux;
use dust_lpc8xx::plu::OutputMux;
use dust_lpc8xx::PLU;

fn delay(n: usize) {
    for _ in 0..n {
        // Make sure the loop is not optimized away
        unsafe { asm!("" :::: "volatile") }
    }
}

const LED: (usize, usize) = (0, 1);

#[no_mangle]
pub fn main() {
    let syscon = unsafe { &mut *SYSCON };
    let swm = unsafe { &mut *SWM };
    let plu = unsafe { &mut *PLU };
    let gpio = unsafe { &mut *GPIO };

    let mut gpio_port = unsafe {
        syscon.enable_clock(syscon::CLOCK_GPIO0);
        let (port_index, bit_index) = LED;
        let mut port = gpio::Port::new(gpio, port_index);
        port.dir_set(1 << bit_index);
        port
    };

    // initialize PLU
    unsafe {
        syscon.enable_clock(syscon::CLOCK_PLU);

        // mux primary input 2 to LUT 0 input 0
        plu.set_lut_input(0, 0, LutInputMux::PluInput(2));
        // put LUT input 0 through to LUT output for all LUTs
        for i in 0..26 {
            plu.lut_truth[i].write(0xaaaaaaaa);
        }
        // mux output of LUT i to LUT i+1 input 0
        for i in 0..25 {
            plu.set_lut_input(i + 1, 0, LutInputMux::LutOutput(i));
        }
        // put LUT 1 bit 0 through to LUT 0 output
        // mux the output of LUT 0 to module outputs 1 and 2
        plu.set_output(1, OutputMux::LutOutput(0));
        plu.set_output(2, OutputMux::LutOutput(0));
        // mux the output of LUT 1 to module output 6
        plu.set_output(6, OutputMux::LutOutput(10));

        syscon.disable_clock(syscon::CLOCK_PLU);
    }

    // initialize SWM
    unsafe {
        syscon.enable_clock(syscon::CLOCK_SWM);

        swm.set_movable_fixed_function_pin(swm::PLU_INPUT2, 10);
        swm.set_movable_fixed_function_pin(swm::PLU_OUT1, 15);
        swm.set_movable_fixed_function_pin(swm::PLU_OUT2, 16);
        swm.set_movable_fixed_function_pin(swm::PLU_OUT6, 13);

        syscon.disable_clock(syscon::CLOCK_SWM);
    }

    let d = 500000;
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
