//! SWM peripheral unit for LPC81x
//!
//! Checked against UM10601 2014-04-02.

use crate::swm::{FixedFunction, MovableFunction};

pub const PINASSIGN_REGISTERS: usize = 9;
pub const PINASSIGN_FIXED_REGISTERS: usize = 0;
pub const PINENABLE_REGISTERS: usize = 1;

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
movable_function!(SPI0_SSEL, 4, 2);
movable_function!(SPI1_SCK, 4, 3);

movable_function!(SPI1_MOSI, 5, 0);
movable_function!(SPI1_MISO, 5, 1);
movable_function!(SPI1_SSEL0, 5, 2);
movable_function!(SCT_PIN0, 5, 3);

movable_function!(SCT_PIN1, 6, 0);
movable_function!(SCT_PIN2, 6, 1);
movable_function!(SCT_PIN3, 6, 2);
movable_function!(SCT_OUT0, 6, 3);

movable_function!(SCT_OUT1, 7, 0);
movable_function!(SCT_OUT2, 7, 1);
movable_function!(SCT_OUT3, 7, 2);
movable_function!(I2C_SDA, 7, 3);

movable_function!(I2C_SCL, 8, 0);
movable_function!(ACMP_O, 8, 1);
movable_function!(CLKOUT, 8, 2);
movable_function!(GPIO_INT_BMAT, 8, 3);

fixed_pin_function!(ACMP_I1, 0, 0);
fixed_pin_function!(ACMP_I2, 0, 1);
fixed_pin_function!(SWCLK, 0, 2);
fixed_pin_function!(SWDIO, 0, 3);
fixed_pin_function!(XTALIN, 0, 4);
fixed_pin_function!(XTALOUT, 0, 5);
fixed_pin_function!(RESETN, 0, 6);
fixed_pin_function!(CLKIN, 0, 7);
fixed_pin_function!(VDDCMP, 0, 8);

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
