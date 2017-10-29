use volatile_register::RW;


#[repr(C)]
pub struct Iocon {
    pub iocon: [RW<u32>; 0x0b0 / 4],
    pub sck_loc: RW<u32>,
    pub dsr_loc: RW<u32>,
    pub dcd_loc: RW<u32>,
    pub ri_loc: RW<u32>,
}


#[derive(Clone, Copy, Debug)]
pub struct Pin(usize);

pub const PIO2_6: Pin = Pin(0);
pub const PIO2_0: Pin = Pin(2);
pub const RESET_PIO0_0: Pin = Pin(3);
pub const PIO0_1: Pin = Pin(4);
pub const PIO1_8: Pin = Pin(5);
pub const PIO0_2: Pin = Pin(7);
pub const PIO2_7: Pin = Pin(8);
pub const PIO2_8: Pin = Pin(9);
pub const PIO2_1: Pin = Pin(10);
pub const PIO0_3: Pin = Pin(11);
pub const PIO0_4: Pin = Pin(12);
pub const PIO0_5: Pin = Pin(13);
pub const PIO1_9: Pin = Pin(14);
pub const PIO3_4: Pin = Pin(15);
pub const PIO2_4: Pin = Pin(16);
pub const PIO2_5: Pin = Pin(17);
pub const PIO3_5: Pin = Pin(18);
pub const PIO0_6: Pin = Pin(19);
pub const PIO0_7: Pin = Pin(20);
pub const PIO2_9: Pin = Pin(21);
pub const PIO2_10: Pin = Pin(22);
pub const PIO2_2: Pin = Pin(23);
pub const PIO0_8: Pin = Pin(24);
pub const PIO0_9: Pin = Pin(25);
pub const SWCLK_PIO0_10: Pin = Pin(26);
pub const PIO1_10: Pin = Pin(27);
pub const PIO2_11: Pin = Pin(28);
pub const PIO0_11: Pin = Pin(29);
pub const PIO1_0: Pin = Pin(30);
pub const PIO1_1: Pin = Pin(31);
pub const PIO1_2: Pin = Pin(32);
pub const PIO3_0: Pin = Pin(33);
pub const PIO3_1: Pin = Pin(34);
pub const PIO2_3: Pin = Pin(35);
pub const SWDIO_PIO1_3: Pin = Pin(36);
pub const PIO1_4: Pin = Pin(37);
pub const PIO1_11: Pin = Pin(38);
pub const PIO3_2: Pin = Pin(39);
pub const PIO1_5: Pin = Pin(40);
pub const PIO1_6: Pin = Pin(41);
pub const PIO1_7: Pin = Pin(42);
pub const PIO3_3: Pin = Pin(43);


pub const IOCON_MODE_INACTIVE: u32 = 0 << 3;
pub const IOCON_MODE_PULL_DOWN: u32 = 1 << 3;
pub const IOCON_MODE_PULL_UP: u32 = 2 << 3;
pub const IOCON_MODE_REPEATER: u32 = 3 << 3;

pub const IOCON_HYS: u32 = 1 << 5;

pub const IOCON_ADMODE_ANALOG: u32 = 0 << 7;
pub const IOCON_ADMODE_DIGITAL: u32 = 1 << 7;

pub const IOCON_I2CMODE_STANDARD_FAST: u32 = 0 << 8;
pub const IOCON_I2CMODE_STANDARD_IO: u32 = 1 << 8;
pub const IOCON_I2CMODE_FAST_MODE_PLUS: u32 = 2 << 8;

pub const IOCON_OD: u32 = 1 << 10;


impl Iocon {
    pub unsafe fn write(&self, pin: Pin, v: u32) {
        self.iocon[pin.0].write(v);
    }
    pub fn set_od_on(&self, pin: Pin) {
        unsafe {
            self.iocon[pin.0].modify(|w| w | IOCON_OD);
        }
    }
    pub fn set_od_off(&self, pin: Pin) {
        unsafe {
            self.iocon[pin.0].modify(|w| w & !IOCON_OD);
        }
    }
}


#[cfg(test)]
mod test {
    #[test]
    fn test_iocon() {
        let iocon = unsafe { &mut *::IOCON };

        assert_eq!(address(&iocon.iocon), 0x4004_4000);
        assert_eq!(address(&iocon.sck_loc), 0x4004_40b0);
        assert_eq!(address(&iocon.dsr_loc), 0x4004_40b4);
        assert_eq!(address(&iocon.dcd_loc), 0x4004_40b8);
        assert_eq!(address(&iocon.ri_loc), 0x4004_40bc);
    }

    fn address<T>(r: *const T) -> usize {
        r as usize
    }
}
