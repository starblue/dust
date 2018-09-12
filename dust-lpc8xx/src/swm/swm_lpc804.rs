//! SWM peripheral unit for LPC804
//!
//! Checked against UM11065 2018-03-21.

use crate::swm::{FixedFunction, MovableFixedFunction, MovableFunction};

pub const PINASSIGN_REGISTERS: usize = 10;
pub const PINASSIGN_FIXED_REGISTERS: usize = 1;
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
movable_function!(I2C1_SDA, 7, 1);
movable_function!(I2C1_SCL, 7, 2);
movable_function!(PLU_CLKIN, 7, 3);

movable_function!(CAPT_X0, 8, 0);
movable_function!(CAPT_X1, 8, 1);
movable_function!(CAPT_X2, 8, 2);
movable_function!(CAPT_X3, 8, 3);

movable_function!(CAPT_X4, 9, 0);
movable_function!(CAPT_YL, 9, 1);
movable_function!(CAPT_YH, 9, 2);

movable_fixed_function!(PLU_INPUT0, 0, 0 => 0, 8, 17);
movable_fixed_function!(PLU_INPUT1, 0, 1 => 1, 9, 18);
movable_fixed_function!(PLU_INPUT2, 0, 2 => 2, 10, 19);
movable_fixed_function!(PLU_INPUT3, 0, 3 => 3, 11, 20);
movable_fixed_function!(PLU_INPUT4, 0, 4 => 4, 12, 21);
movable_fixed_function!(PLU_INPUT5, 0, 5 => 5, 13, 22);
movable_fixed_function!(PLU_OUT0, 0, 6 => 7, 14, 23);
movable_fixed_function!(PLU_OUT1, 0, 7 => 8, 15, 24);
movable_fixed_function!(PLU_OUT2, 0, 8 => 9, 16, 25);
movable_fixed_function!(PLU_OUT3, 0, 9 => 10, 17, 26);
movable_fixed_function!(PLU_OUT4, 0, 10 => 11, 18, 27);
movable_fixed_function!(PLU_OUT5, 0, 11 => 12, 19, 28);
movable_fixed_function!(PLU_OUT6, 0, 12 => 13, 20, 29);
movable_fixed_function!(PLU_OUT7, 0, 13 => 14, 21, 30);

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
fixed_pin_function!(ACMP_I5, 0, 22);
fixed_pin_function!(DACOUT0, 0, 23);

#[cfg(test)]
mod test {
    #[test]
    fn test_swm() {
        let swm = unsafe { &mut *::SWM };

        assert_eq!(address(&swm.pinassign[0]), 0x4000_c000);
        assert_eq!(address(&swm.pinassign_fixed[0]), 0x4000_c180);
        assert_eq!(address(&swm.pinenable[0]), 0x4000_c1c0);
    }

    fn address<T>(r: *const T) -> usize {
        r as usize
    }
}
