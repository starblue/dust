use volatile_register::RW;

#[derive(Clone, Copy, Debug)]
pub struct MovableFunction(usize);

const PINS: usize = ::gpio::PINS;
const MOVABLE_FUNCTIONS: usize = 48;
const PINASSIGN_REGISTERS: usize = (MOVABLE_FUNCTIONS + 3) / 4;

// PINASSIGN0
pub const U0_TXD: MovableFunction = MovableFunction(0);
pub const U0_RXD: MovableFunction = MovableFunction(1);
pub const U0_RTS: MovableFunction = MovableFunction(2);
pub const U0_CTS: MovableFunction = MovableFunction(3);

// PINASSIGN1
pub const U0_SCLK: MovableFunction = MovableFunction(4);
pub const U1_TXD: MovableFunction = MovableFunction(5);
pub const U1_RXD: MovableFunction = MovableFunction(6);
pub const U1_RTS: MovableFunction = MovableFunction(7);

// PINASSIGN2
pub const U1_CTS: MovableFunction = MovableFunction(8);
pub const U1_SCLK: MovableFunction = MovableFunction(9);
pub const U2_TXD: MovableFunction = MovableFunction(10);
pub const U2_RXD: MovableFunction = MovableFunction(11);

// PINASSIGN3
pub const U2_RTS: MovableFunction = MovableFunction(12);
pub const U2_CTS: MovableFunction = MovableFunction(13);
pub const U2_SCLK: MovableFunction = MovableFunction(14);
pub const SPI0_SCK: MovableFunction = MovableFunction(15);

// PINASSIGN4
pub const SPI0_MOSI: MovableFunction = MovableFunction(16);
pub const SPI0_MISO: MovableFunction = MovableFunction(17);
pub const SPI0_SSEL0: MovableFunction = MovableFunction(18);
pub const SPI0_SSEL1: MovableFunction = MovableFunction(19);

// PINASSIGN5
pub const SPI0_SSEL2: MovableFunction = MovableFunction(20);
pub const SPI0_SSEL3: MovableFunction = MovableFunction(21);
pub const SPI1_SCK: MovableFunction = MovableFunction(22);
pub const SPI1_MOSI: MovableFunction = MovableFunction(23);

// PINASSIGN6
pub const SPI1_MISO: MovableFunction = MovableFunction(24);
pub const SPI1_SSEL0: MovableFunction = MovableFunction(25);
pub const SPI1_SSEL1: MovableFunction = MovableFunction(26);
pub const SCT_PIN0: MovableFunction = MovableFunction(27);

// PINASSIGN7
pub const SCT_PIN1: MovableFunction = MovableFunction(28);
pub const SCT_PIN2: MovableFunction = MovableFunction(29);
pub const SCT_PIN3: MovableFunction = MovableFunction(30);
pub const SCT_OUT0: MovableFunction = MovableFunction(31);

// PINASSIGN8
pub const SCT_OUT1: MovableFunction = MovableFunction(32);
pub const SCT_OUT2: MovableFunction = MovableFunction(33);
pub const SCT_OUT3: MovableFunction = MovableFunction(34);
pub const SCT_OUT4: MovableFunction = MovableFunction(35);

// PINASSIGN9
pub const SCT_OUT5: MovableFunction = MovableFunction(36);
pub const I2C1_SDA: MovableFunction = MovableFunction(37);
pub const I2C1_SCL: MovableFunction = MovableFunction(38);
pub const I2C2_SDA: MovableFunction = MovableFunction(39);

// PINASSIGN10
pub const I2C2_SCL: MovableFunction = MovableFunction(40);
pub const I2C3_SDA: MovableFunction = MovableFunction(41);
pub const I2C3_SCL: MovableFunction = MovableFunction(42);
pub const ADC_PINTRIG0: MovableFunction = MovableFunction(43);

// PINASSIGN11
pub const ADC_PINTRIG1: MovableFunction = MovableFunction(44);
pub const ACMP_O: MovableFunction = MovableFunction(45);
pub const CLKOUT: MovableFunction = MovableFunction(46);
pub const GPIO_INT_BMAT: MovableFunction = MovableFunction(47);

#[derive(Clone, Copy, Debug)]
pub struct FixedFunction(usize);

const FIXED_FUNCTIONS: usize = 25;
const PINENABLE_REGISTERS: usize = (FIXED_FUNCTIONS + 31) / 32;

pub const ACMP_I1: FixedFunction = FixedFunction(0);
pub const ACMP_I2: FixedFunction = FixedFunction(1);
pub const ACMP_I3: FixedFunction = FixedFunction(2);
pub const ACMP_I4: FixedFunction = FixedFunction(3);
pub const SWCLK: FixedFunction = FixedFunction(4);
pub const SWDIO: FixedFunction = FixedFunction(5);
pub const XTALIN: FixedFunction = FixedFunction(6);
pub const XTALOUT: FixedFunction = FixedFunction(7);
pub const RESETN: FixedFunction = FixedFunction(8);
pub const CLKIN: FixedFunction = FixedFunction(9);
pub const VDDCMP: FixedFunction = FixedFunction(10);
pub const I2C0_SDA: FixedFunction = FixedFunction(11);
pub const I2C0_SCL: FixedFunction = FixedFunction(12);
pub const ADC0: FixedFunction = FixedFunction(13);
pub const ADC1: FixedFunction = FixedFunction(14);
pub const ADC2: FixedFunction = FixedFunction(15);
pub const ADC3: FixedFunction = FixedFunction(16);
pub const ADC4: FixedFunction = FixedFunction(17);
pub const ADC5: FixedFunction = FixedFunction(18);
pub const ADC6: FixedFunction = FixedFunction(19);
pub const ADC7: FixedFunction = FixedFunction(20);
pub const ADC8: FixedFunction = FixedFunction(21);
pub const ADC9: FixedFunction = FixedFunction(22);
pub const ADC10: FixedFunction = FixedFunction(23);
pub const ADC11: FixedFunction = FixedFunction(24);

#[repr(C)]
pub struct Swm {
    pub pinassign: [RW<u32>; PINASSIGN_REGISTERS],
    reserved0: [u8; 0x1c0 - 4 * PINASSIGN_REGISTERS],
    pub pinenable: [RW<u32>; PINENABLE_REGISTERS],
}

impl MovableFunction {
    pub fn value(self) -> usize {
        self.0
    }
}

impl FixedFunction {
    pub fn value(self) -> usize {
        self.0
    }
}

impl Swm {
    pub unsafe fn enable_fixed_function(&mut self, f: FixedFunction) {
        let (word, bit) = word_bit(f);
        self.pinenable[word].modify(|b| b & !(1 << bit));
    }
    pub unsafe fn disable_fixed_function(&mut self, f: FixedFunction) {
        let (word, bit) = word_bit(f);
        self.pinenable[word].modify(|b| b | (1 << bit));
    }

    pub unsafe fn disable_movable_function(&mut self, f: MovableFunction) {
        self.set_movable_function_pin_internal(f, 0xff);
    }
    pub unsafe fn set_movable_function_pin(
        &self,
        f: MovableFunction,
        pin: usize,
    ) {
        // TODO use assertion
        if pin >= PINS {
            panic!();
        }
        self.set_movable_function_pin_internal(f, pin);
    }
    unsafe fn set_movable_function_pin_internal(
        &self,
        f: MovableFunction,
        pin: usize,
    ) {
        let f = f.value();
        // TODO use assertion
        if f >= MOVABLE_FUNCTIONS {
            panic!();
        }
        let r = f / 4;
        let shift = 8 * (r % 4);
        let mask: u32 = 0xff << shift;
        let data: u32 = (pin as u32) << shift;
        self.pinassign[r].modify(|w| (w & !mask) | data);
    }
}

fn word_bit(f: FixedFunction) -> (usize, usize) {
    let v = f.value();
    if v >= FIXED_FUNCTIONS {
        panic!();
    }
    let word = v / 32;
    let bit = v % 32;
    (word, bit)
}


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
