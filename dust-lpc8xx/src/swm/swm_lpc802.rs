//! SWM peripheral unit for LPC802
//!
//! Checked against UM11045 2018-03-23.

use crate::swm::{FixedFunction, MovableFunction};

pub const PINASSIGN_REGISTERS: usize = 8;
pub const PINASSIGN_FIXED_REGISTERS: usize = 0;
pub const PINENABLE_REGISTERS: usize = 1;

movable_function!(U0_TXD, 0, 0);
movable_function!(U0_RXD, 0, 1);
movable_function!(U0_RTS, 0, 2);
movable_function!(U0_CTS, 0, 3);

movable_function!(U0_SCLK, 1, 0);
movable_function!(U1_TXD, 1, 1);
movable_function!(U1_RXD, 1, 2);
movable_function!(U1_SCLK, 1, 3);

movable_function!(SPI0_SCK, 2, 0);
movable_function!(SPI0_MOSI, 2, 1);
movable_function!(SPI0_MISO, 2, 2);
movable_function!(SPI0_SSEL0, 2, 3);

movable_function!(SPI0_SSEL1, 3, 0);
movable_function!(T0_CAP0, 3, 1);
movable_function!(T0_CAP1, 3, 2);
movable_function!(T0_CAP2, 3, 3);

movable_function!(T0_MAT0, 4, 0);
movable_function!(T0_MAT1, 4, 1);
movable_function!(T0_MAT2, 4, 2);
movable_function!(T0_MAT3, 4, 3);

movable_function!(I2C0_SDA, 5, 0);
movable_function!(I2C0_SCL, 5, 1);
movable_function!(COMP0_OUT, 5, 2);
movable_function!(CLKOUT, 5, 3);

movable_function!(GPIO_INT_BMAT, 6, 0);
movable_function!(LVLSHFT_IN0, 6, 1);
movable_function!(LVLSHFT_IN1, 6, 2);
movable_function!(LVLSHFT_OUT0, 6, 3);

movable_function!(LVLSHFT_OUT1, 7, 0);

fixed_pin_function!(ACMP_I1, 0, 0);
fixed_pin_function!(ACMP_I2, 0, 1);
fixed_pin_function!(ACMP_I3, 0, 2);
fixed_pin_function!(ACMP_I4, 0, 3);
fixed_pin_function!(SWCLK, 0, 4);
fixed_pin_function!(SWDIO, 0, 5);
fixed_pin_function!(RESETN, 0, 6);
fixed_pin_function!(CLKIN, 0, 7);
fixed_pin_function!(WKCLKIN, 0, 8);
fixed_pin_function!(VDDCMP, 0, 9);
fixed_pin_function!(ADC_0, 0, 10);
fixed_pin_function!(ADC_1, 0, 11);
fixed_pin_function!(ADC_2, 0, 12);
fixed_pin_function!(ADC_3, 0, 13);
fixed_pin_function!(ADC_4, 0, 14);
fixed_pin_function!(ADC_5, 0, 15);
fixed_pin_function!(ADC_6, 0, 16);
fixed_pin_function!(ADC_7, 0, 17);
fixed_pin_function!(ADC_8, 0, 18);
fixed_pin_function!(ADC_9, 0, 19);
fixed_pin_function!(ADC_10, 0, 20);
fixed_pin_function!(ADC_11, 0, 21);

#[cfg(test)]
mod test {
    #[test]
    fn test_swm() {
        let swm = unsafe { &mut *::SWM };

        assert_eq!(address(&swm.pinassign[0]), 0x4000_c000);
        assert_eq!(address(&swm.pinenable[0]), 0x4000_c1c0);
    }

    fn address<T>(r: *const T) -> usize {
        r as usize
    }
}
