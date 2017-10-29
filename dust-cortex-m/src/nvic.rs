use volatile_register::{RO, RW};

#[repr(C)]
/// Nested Vectored Interrupt Controller
pub struct Nvic {
    /// Interrupt Set-Enable
    pub iser: [RW<u32>; 16],
    reserved0: [u32; 16],
    /// Interrupt Clear-Enable
    pub icer: [RW<u32>; 16],
    reserved1: [u32; 16],
    /// Interrupt Set-Pending
    pub ispr: [RW<u32>; 16],
    reserved2: [u32; 16],
    /// Interrupt Clear-Pending
    pub icpr: [RW<u32>; 16],
    reserved3: [u32; 16],
    /// Interrupt Active Bit
    pub iabr: [RO<u32>; 16],
    reserved4: [u32; 48],
    /// Interrupt Priority
    pub ipr: [RW<u32>; 124],
}


impl Nvic {
    pub fn enable_irq(&mut self, n: usize) {
        unsafe {
            self.iser[n / 32].write(1 << (n % 32));
        }
    }
    pub fn disable_irq(&mut self, n: usize) {
        unsafe {
            self.icer[n / 32].write(1 << (n % 32));
        }
    }
    pub fn set_irq_priority(&mut self, n: usize, prio: u8) {
        let reg = n / 4;
        let shift = 8 * (n % 4);
        let mask = 0xff << shift;
        unsafe {
            self.ipr[reg].modify(|w| (w & !mask) | (prio as u32) << shift);
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_nvic() {
        let nvic = unsafe { &mut *::NVIC };

        assert_eq!(address(&nvic.iser), 0xE000_E100);
        assert_eq!(address(&nvic.icer), 0xE000_E180);
        assert_eq!(address(&nvic.ispr), 0xE000_E200);
        assert_eq!(address(&nvic.icpr), 0xE000_E280);
        assert_eq!(address(&nvic.iabr), 0xE000_E300);
        assert_eq!(address(&nvic.ipr), 0xE000_E400);
    }

    fn address<T>(r: *const T) -> usize {
        r as usize
    }
}
