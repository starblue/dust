use dust::gpio::port;
use volatile_register::{RO, RW, WO};

#[repr(C)]
pub struct Gpio {
    pub moder: RW<u32>,
    pub otyper: RW<u32>,
    pub ospeedr: RW<u32>,
    pub pupdr: RW<u32>,
    pub idr: RO<u32>,
    pub odr: RW<u32>,
    pub bsrr: WO<u32>,
    pub lckr: RW<u32>,
    pub afr: [RW<u32>; 2],
    pub brr: WO<u32>,
}

pub struct Port<'a> {
    gpio: &'a Gpio,
}

impl<'a> Port<'a> {
    pub unsafe fn new(gpio: &mut Gpio) -> Port {
        Port { gpio }
    }
}

impl<'a> port::GetValue<u32> for Port<'a> {
    fn get_value(&mut self) -> u32 {
        self.gpio.idr.read()
    }
}

impl<'a> port::SetValue<u32> for Port<'a> {
    fn set_value(&mut self, value: u32) {
        unsafe { self.gpio.odr.write(value) }
    }
    fn modify_value<F>(&mut self, f: F)
    where
        F: FnMut(u32) -> u32,
    {
        unsafe {
            self.gpio.odr.modify(f);
        }
    }
}

impl<'a> port::Set<u32> for Port<'a> {
    fn set_bits(&mut self, bits: u32) {
        unsafe {
            self.gpio.bsrr.write(bits & 0xffff);
        }
    }
}

impl<'a> port::Clr<u32> for Port<'a> {
    fn clr_bits(&mut self, bits: u32) {
        unsafe {
            self.gpio.brr.write(bits & 0xffff);
        }
    }
}

impl Gpio {
    pub fn set_pin_output(&self, pin: usize) {
        unsafe {
            let shift = 2 * pin;
            let mask = 3 << shift;
            let value = 1 << shift;
            self.moder.modify(|w| (w & !mask) | value);
        }
    }
    pub fn set_pin_input(&self, pin: usize) {
        unsafe {
            let shift = 2 * pin;
            let mask = 3 << shift;
            let value = 0 << shift;
            self.moder.modify(|w| (w & !mask) | value);
        }
    }
    pub fn get_pin_value(&self, pin: usize) -> bool {
        (self.idr.read() & (1 << pin)) != 0
    }
    pub fn set_pin(&self, pin: usize) {
        unsafe {
            self.bsrr.write(1 << pin);
        }
    }
    pub fn clr_pin(&self, pin: usize) {
        unsafe {
            self.brr.write(1 << pin);
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_gpio() {
        use crate::GPIOA;
        use crate::GPIOB;
        use crate::GPIOC;
        use crate::GPIOD;
        use crate::GPIOF;

        let gpio = unsafe { &mut *GPIOA };

        assert_eq!(address(&gpio.moder), 0x4800_0000);
        assert_eq!(address(&gpio.otyper), 0x4800_0004);
        assert_eq!(address(&gpio.ospeedr), 0x4800_0008);
        assert_eq!(address(&gpio.pupdr), 0x4800_000c);
        assert_eq!(address(&gpio.idr), 0x4800_0010);
        assert_eq!(address(&gpio.odr), 0x4800_0014);
        assert_eq!(address(&gpio.bsrr), 0x4800_0018);
        assert_eq!(address(&gpio.lckr), 0x4800_001c);
        assert_eq!(address(&gpio.afr), 0x4800_0020);
        assert_eq!(address(&gpio.brr), 0x4800_0028);

        let gpiob = unsafe { &mut *GPIOB };
        assert_eq!(address(&gpiob.moder), 0x4800_0400);

        let gpioc = unsafe { &mut *GPIOC };
        assert_eq!(address(&gpioc.moder), 0x4800_0800);

        let gpiod = unsafe { &mut *GPIOD };
        assert_eq!(address(&gpiod.moder), 0x4800_0c00);

        let gpiof = unsafe { &mut *GPIOF };
        assert_eq!(address(&gpiof.moder), 0x4800_1400);
    }

    fn address<T>(r: *const T) -> usize {
        r as usize
    }
}
