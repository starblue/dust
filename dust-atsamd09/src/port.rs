use dust::gpio::port;
use volatile_register::{RO, RW, WO};

#[repr(C)]
pub struct Regs {
    pub dir: RW<u32>,
    pub dirclr: RW<u32>,
    pub dirset: RW<u32>,
    pub dirtgl: RW<u32>,
    pub out: RW<u32>,
    pub outclr: RW<u32>,
    pub outset: RW<u32>,
    pub outtgl: RW<u32>,
    pub in_: RO<u32>,
    pub ctrl: RW<u32>,
    pub wrconfig: WO<u32>,
    reserved1: [u8; 0x30 - 0x2c],
    pub pmux: [RW<u8>; 16],
    pub pincfg: [RW<u8>; 32],
}

pub struct Port<'a> {
    regs: &'a Regs,
}

impl<'a> Port<'a> {
    pub unsafe fn new(regs: &mut Regs) -> Port {
        Port { regs }
    }
}

impl<'a> port::GetValue<u32> for Port<'a> {
    fn get_value(&mut self) -> u32 {
        self.regs.in_.read()
    }
}

impl<'a> port::SetValue<u32> for Port<'a> {
    fn set_value(&mut self, value: u32) {
        unsafe { self.regs.out.write(value) }
    }
    fn modify_value<F>(&mut self, f: F)
    where
        F: FnMut(u32) -> u32,
    {
        unsafe {
            self.regs.out.modify(f);
        }
    }
}

impl<'a> port::Set<u32> for Port<'a> {
    fn set_bits(&mut self, bits: u32) {
        unsafe {
            self.regs.outset.write(bits);
        }
    }
}

impl<'a> port::Clr<u32> for Port<'a> {
    fn clr_bits(&mut self, bits: u32) {
        unsafe {
            self.regs.outclr.write(bits);
        }
    }
}

impl<'a> port::Toggle<u32> for Port<'a> {
    fn toggle_bits(&mut self, bits: u32) {
        unsafe {
            self.regs.outtgl.write(bits);
        }
    }
}

impl<'a> port::DirSetValue<u32> for Port<'a> {
    fn dir_set_value(&mut self, dir: u32) {
        unsafe {
            self.regs.dir.write(dir);
        }
    }
    fn dir_modify_value<F>(&mut self, f: F)
    where
        F: FnMut(u32) -> u32,
    {
        unsafe {
            self.regs.dir.modify(f);
        }
    }
}

impl<'a> port::DirSet<u32> for Port<'a> {
    fn dir_set(&mut self, dir: u32) {
        unsafe {
            self.regs.dirset.write(dir);
        }
    }
}

impl<'a> port::DirClr<u32> for Port<'a> {
    fn dir_clr(&mut self, dir: u32) {
        unsafe {
            self.regs.dirclr.write(dir);
        }
    }
}

impl<'a> port::DirToggle<u32> for Port<'a> {
    fn dir_toggle(&mut self, dir: u32) {
        unsafe {
            self.regs.dirtgl.write(dir);
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_gpio() {
        let port = unsafe { &mut *::PORT };

        assert_eq!(address(&port.dir), 0x41004400);
        assert_eq!(address(&port.dirclr), 0x41004404);
        assert_eq!(address(&port.dirset), 0x41004408);
        assert_eq!(address(&port.dirtgl), 0x4100440c);
        assert_eq!(address(&port.out), 0x41004410);
        assert_eq!(address(&port.outclr), 0x41004414);
        assert_eq!(address(&port.outset), 0x41004418);
        assert_eq!(address(&port.outtgl), 0x4100441c);
        assert_eq!(address(&port.in_), 0x41004420);
        assert_eq!(address(&port.ctrl), 0x41004424);
        assert_eq!(address(&port.wrconfig), 0x41004428);
        assert_eq!(address(&port.pmux), 0x41004430);
        assert_eq!(address(&port.pincfg), 0x41004440);
    }

    fn address<T>(r: *const T) -> usize {
        r as usize
    }
}
