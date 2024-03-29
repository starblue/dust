use dust::gpio::port;
use volatile_register::{RO, RW, WO};

#[repr(C)]
pub struct Gpio {
    pub data: [RW<u32>; 0x1000],
    reserved0: [u32; 0x1000],
    pub dir: RW<u32>,
    pub is: RW<u32>,
    pub ibe: RW<u32>,
    pub iev: RW<u32>,
    pub ie: RW<u32>,
    pub ris: RO<u32>,
    pub mis: RO<u32>,
    pub ic: WO<u32>,
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
        self.gpio.data[0xfff].read()
    }
}

impl<'a> port::SetValue<u32> for Port<'a> {
    fn set_value(&mut self, value: u32) {
        unsafe { self.gpio.data[0xfff].write(value) }
    }
    fn modify_value<F>(&mut self, f: F)
    where
        F: FnMut(u32) -> u32,
    {
        unsafe {
            self.gpio.data[0xfff].modify(f);
        }
    }
}

impl<'a> port::Set<u32> for Port<'a> {
    fn set_bits(&mut self, bits: u32) {
        let i = (bits & 0xfff) as usize;
        unsafe {
            self.gpio.data[i].write(!0);
        }
    }
}

impl<'a> port::Clr<u32> for Port<'a> {
    fn clr_bits(&mut self, bits: u32) {
        let i = (bits & 0xfff) as usize;
        unsafe {
            self.gpio.data[i].write(0);
        }
    }
}

impl<'a> port::DirSetValue<u32> for Port<'a> {
    fn dir_set_value(&mut self, dir: u32) {
        unsafe {
            self.gpio.dir.write(dir);
        }
    }
    fn dir_modify_value<F>(&mut self, f: F)
    where
        F: FnMut(u32) -> u32,
    {
        unsafe {
            self.gpio.dir.modify(f);
        }
    }
}

impl Gpio {
    pub fn set_pin_output(&self, pin: usize) {
        unsafe {
            self.dir.modify(|w| w | (1 << pin));
        }
    }
    pub fn set_pin_input(&self, pin: usize) {
        unsafe {
            self.dir.modify(|w| w & !(1 << pin));
        }
    }
    pub fn set_pin(&self, pin: usize) {
        unsafe {
            self.data[1 << pin].write(!0);
        }
    }
    pub fn clr_pin(&self, pin: usize) {
        unsafe {
            self.data[1 << pin].write(0);
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_gpio() {
        let gpio0 = unsafe { &mut *::GPIO0 };
        let gpio1 = unsafe { &mut *::GPIO1 };
        let gpio2 = unsafe { &mut *::GPIO2 };
        let gpio3 = unsafe { &mut *::GPIO3 };

        assert_eq!(address(&gpio0.data), 0x5000_0000);
        assert_eq!(address(&gpio0.dir), 0x5000_8000);
        assert_eq!(address(&gpio0.is), 0x5000_8004);
        assert_eq!(address(&gpio0.ibe), 0x5000_8008);
        assert_eq!(address(&gpio0.iev), 0x5000_800c);
        assert_eq!(address(&gpio0.ie), 0x5000_8010);
        assert_eq!(address(&gpio0.ris), 0x5000_8014);
        assert_eq!(address(&gpio0.mis), 0x5000_8018);
        assert_eq!(address(&gpio0.ic), 0x5000_801c);

        assert_eq!(address(&gpio1.data), 0x5001_0000);
        assert_eq!(address(&gpio2.data), 0x5002_0000);
        assert_eq!(address(&gpio3.data), 0x5003_0000);
    }

    fn address<T>(r: *const T) -> usize {
        r as usize
    }
}
