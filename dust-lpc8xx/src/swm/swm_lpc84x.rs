//! SWM peripheral unit for LPC84x
//!
//! TODO Checked against UM11029 2017-12-08.

use swm::{FixedFunction, MovableFunction};

pub const PINASSIGN_REGISTERS: usize = 15;
pub const PINASSIGN_FIXED_REGISTERS: usize = 0;
pub const PINENABLE_REGISTERS: usize = 2;

movable_function!(U0_TXD, 0, 0);
movable_function!(U0_RXD, 0, 1);
movable_function!(U0_RTS, 0, 2);
movable_function!(U0_CTS, 0, 3);

movable_function!(U0_SCLK, 1, 0);
movable_function!(U1_TXD, 1, 1);
movable_function!(U1_RXD, 1, 2);
movable_function!(U1_RTS, 1, 3);

movable_function!(U1_CTS, 2, 0);
movable_function!(U1_SCLK, 2, 1);
movable_function!(U2_TXD, 2, 2);
movable_function!(U2_RXD, 2, 3);

movable_function!(U2_RTS, 3, 0);
movable_function!(U2_CTS, 3, 1);
movable_function!(U2_SCLK, 3, 2);
movable_function!(SPI0_SCK, 3, 3);

movable_function!(SPI0_MOSI, 4, 0);
movable_function!(SPI0_MISO, 4, 1);
movable_function!(SPI0_SSEL0, 4, 2);
movable_function!(SPI0_SSEL1, 4, 3);

movable_function!(SPI0_SSEL2, 5, 0);
movable_function!(SPI0_SSEL3, 5, 1);
movable_function!(SPI1_SCK, 5, 2);
movable_function!(SPI1_MOSI, 5, 3);

movable_function!(SPI1_MISO, 6, 0);
movable_function!(SPI1_SSEL0, 6, 1);
movable_function!(SPI1_SSEL1, 6, 2);
movable_function!(SCT_PIN0, 6, 3);

movable_function!(SCT_PIN1, 7, 0);
movable_function!(SCT_PIN2, 7, 1);
movable_function!(SCT_PIN3, 7, 2);
movable_function!(SCT_OUT0, 7, 3);

movable_function!(SCT_OUT1, 8, 0);
movable_function!(SCT_OUT2, 8, 1);
movable_function!(SCT_OUT3, 8, 2);
movable_function!(SCT_OUT4, 8, 3);

movable_function!(SCT_OUT5, 9, 0);
movable_function!(SCT_OUT6, 9, 1);
movable_function!(I2C1_SDA, 9, 2);
movable_function!(I2C1_SCL, 9, 3);

movable_function!(I2C2_SDA, 10, 0);
movable_function!(I2C2_SCL, 10, 1);
movable_function!(I2C3_SDA, 10, 2);
movable_function!(I2C3_SCL, 10, 3);

movable_function!(COMP0_OUT0, 11, 0);
movable_function!(CLKOUT, 11, 1);
movable_function!(GPIO_INT_BMAT, 11, 2);
movable_function!(UART3_TXD, 11, 3);

movable_function!(UART3_RXD, 12, 0);
movable_function!(UART3_SCLK, 12, 1);
movable_function!(UART4_TXD, 12, 2);
movable_function!(UART4_RXD, 12, 3);

movable_function!(UART4_SCLK, 13, 0);
movable_function!(T0_MAT0, 13, 1);
movable_function!(T0_MAT1, 13, 2);
movable_function!(T0_MAT2, 13, 3);

movable_function!(T0_MAT3, 14, 0);
movable_function!(T0_CAP0, 14, 1);
movable_function!(T0_CAP1, 14, 2);
movable_function!(T0_CAP2, 14, 3);

fixed_pin_function!(ACMP_I1, 0, 0);
fixed_pin_function!(ACMP_I2, 0, 1);
fixed_pin_function!(ACMP_I3, 0, 2);
fixed_pin_function!(ACMP_I4, 0, 3);
fixed_pin_function!(ACMP_I5, 0, 4);
fixed_pin_function!(SWCLK, 0, 5);
fixed_pin_function!(SWDIO, 0, 6);
fixed_pin_function!(XTALIN, 0, 7);
fixed_pin_function!(XTALOUT, 0, 8);
fixed_pin_function!(RESETN, 0, 9);
fixed_pin_function!(CLKIN, 0, 10);
fixed_pin_function!(VDDCMP, 0, 11);
fixed_pin_function!(I2C0_SDA, 0, 12);
fixed_pin_function!(I2C0_SCL, 0, 13);
fixed_pin_function!(ADC0, 0, 14);
fixed_pin_function!(ADC1, 0, 15);
fixed_pin_function!(ADC2, 0, 16);
fixed_pin_function!(ADC3, 0, 17);
fixed_pin_function!(ADC4, 0, 18);
fixed_pin_function!(ADC5, 0, 19);
fixed_pin_function!(ADC6, 0, 20);
fixed_pin_function!(ADC7, 0, 21);
fixed_pin_function!(ADC8, 0, 22);
fixed_pin_function!(ADC9, 0, 23);
fixed_pin_function!(ADC10, 0, 24);
fixed_pin_function!(ADC11, 0, 25);
fixed_pin_function!(DACOUT0, 0, 26);
fixed_pin_function!(DACOUT1, 0, 27);
fixed_pin_function!(CAPT_X0, 0, 28);
fixed_pin_function!(CAPT_X1, 0, 29);
fixed_pin_function!(CAPT_X2, 0, 30);
fixed_pin_function!(CAPT_X3, 0, 31);

fixed_pin_function!(CAPT_X4, 1, 0);
fixed_pin_function!(CAPT_X5, 1, 1);
fixed_pin_function!(CAPT_X6, 1, 2);
fixed_pin_function!(CAPT_X7, 1, 3);
fixed_pin_function!(CAPT_X8, 1, 4);
fixed_pin_function!(CAPT_YL, 1, 5);
fixed_pin_function!(CAPT_YH, 1, 6);

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
