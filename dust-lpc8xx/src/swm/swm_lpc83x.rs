//! SWM peripheral unit for LPC83x
//!
//! Checked against UM11021 2016-10-05.

use swm::{FixedFunction, MovableFunction};

pub const PINASSIGN_REGISTERS: usize = 12;
pub const PINASSIGN_FIXED_REGISTERS: usize = 0;
pub const PINENABLE_REGISTERS: usize = 1;

movable_function!(U0_TXD, 0, 0);
movable_function!(U0_RXD, 0, 1);
movable_function!(U0_RTS, 0, 2);
movable_function!(U0_CTS, 0, 3);

movable_function!(U0_SCLK, 1, 0);

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

movable_function!(ADC_PINTRIG0, 10, 3);

movable_function!(ADC_PINTRIG1, 11, 0);
movable_function!(CLKOUT, 11, 2);
movable_function!(GPIO_INT_BMAT, 11, 3);

fixed_pin_function!(SWCLK, 0, 4);
fixed_pin_function!(SWDIO, 0, 5);
fixed_pin_function!(XTALIN, 0, 6);
fixed_pin_function!(XTALOUT, 0, 7);
fixed_pin_function!(RESETN, 0, 8);
fixed_pin_function!(CLKIN, 0, 9);
fixed_pin_function!(I2C0_SDA, 0, 11);
fixed_pin_function!(I2C0_SCL, 0, 12);
fixed_pin_function!(ADC0, 0, 13);
fixed_pin_function!(ADC1, 0, 14);
fixed_pin_function!(ADC2, 0, 15);
fixed_pin_function!(ADC3, 0, 16);
fixed_pin_function!(ADC4, 0, 17);
fixed_pin_function!(ADC5, 0, 18);
fixed_pin_function!(ADC6, 0, 19);
fixed_pin_function!(ADC7, 0, 20);
fixed_pin_function!(ADC8, 0, 21);
fixed_pin_function!(ADC9, 0, 22);
fixed_pin_function!(ADC10, 0, 23);
fixed_pin_function!(ADC11, 0, 24);

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
