use dust_register::Modify;
use dust_register::Read;
use dust_register::Write;
use dust_register::{RO, RW};

#[repr(C)]
/// Nested Vectored Interrupt Controller
pub struct SysTick;
impl SysTick {
    /// Control and Status Register
    pub const fn csr(&self) -> RW<u32> {
        RW::at(0xE000_E010)
    }
    /// Reload Value Register
    pub const fn rvr(&self) -> RW<u32> {
        RW::at(0xE000_E014)
    }
    /// Current Value Register
    pub const fn cvr(&self) -> RW<u32> {
        RW::at(0xE000_E018)
    }
    /// Calibration value register
    pub const fn calib(&self) -> RO<u32> {
        RO::at(0xE000_E010)
    }
}

pub const CSR_ENABLE: u32 = 1 << 0;
pub const CSR_TICKINT: u32 = 1 << 1;
pub const CSR_CLKSOURCE: u32 = 1 << 2;
pub const CSR_COUNTFLAG: u32 = 1 << 16;

pub const CALIB_TENMS_MASK: u32 = 0x00ffffff;
pub const CALIB_SKEW: u32 = 1 << 30;
pub const CALIB_NOREF: u32 = 1 << 31;

impl SysTick {
    pub fn init(&mut self, reload_value: u32) {
        unsafe {
            assert!(reload_value < 1 << 24);
            self.rvr().write(reload_value);
            // Set counter to zero by writing any value
            self.cvr().write(0);
            self.enable();
        }
    }
    pub fn calib_tenms(&self) -> u32 {
        unsafe { self.calib().read() & CALIB_TENMS_MASK }
    }
    pub unsafe fn enable(&mut self) {
        self.csr().modify(|w| w | CSR_ENABLE);
    }
    pub unsafe fn disable(&mut self) {
        self.csr().modify(|w| w & !CSR_ENABLE);
    }
    pub unsafe fn enable_tickint(&mut self) {
        self.csr().modify(|w| w | CSR_TICKINT);
    }
    pub unsafe fn disable_tickint(&mut self) {
        self.csr().modify(|w| w & !CSR_TICKINT);
    }
}

#[cfg(test)]
mod test {
    use crate::SYS_TICK;

    #[test]
    fn test_sys_tick() {
        let sys_tick = unsafe { &mut *SYS_TICK };

        assert_eq!(address(&sys_tick.csr), 0xE000_E010);
        assert_eq!(address(&sys_tick.rvr), 0xE000_E014);
        assert_eq!(address(&sys_tick.cvr), 0xE000_E018);
        assert_eq!(address(&sys_tick.calib), 0xE000_E01C);
    }

    fn address<T>(r: *const T) -> usize {
        r as usize
    }
}
