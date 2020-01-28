use dust_register::Read;
use dust_register::Write;
use dust_register::{RO, RW, WO};

#[repr(C)]
pub struct Usart {
    address: usize,
}

impl Usart {
    pub const fn at(address: usize) -> Usart {
        Usart { address }
    }
    pub const fn cfg(&self) -> RW<u32> {
        RW::at(self.address + 0)
    }
    pub const fn ctl(&self) -> RW<u32> {
        RW::at(self.address + 4)
    }
    pub const fn stat(&self) -> RW<u32> {
        RW::at(self.address + 8)
    }
    pub const fn intenset(&self) -> RW<u32> {
        RW::at(self.address + 12)
    }
    pub const fn intenclr(&self) -> WO<u32> {
        WO::at(self.address + 16)
    }
    pub const fn rxdat(&self) -> RO<u32> {
        RO::at(self.address + 20)
    }
    pub const fn rxdatstat(&self) -> RO<u32> {
        RO::at(self.address + 24)
    }
    pub const fn txdat(&self) -> RW<u32> {
        RW::at(self.address + 28)
    }
    pub const fn brg(&self) -> RW<u32> {
        RW::at(self.address + 32)
    }
    pub const fn intstat(&self) -> RO<u32> {
        RO::at(self.address + 36)
    }
    pub const fn osr(&self) -> RW<u32> {
        RW::at(self.address + 40)
    }
    pub const fn addr(&self) -> RW<u32> {
        RW::at(self.address + 44)
    }
}

pub const CFG_ENABLE: u32 = 1 << 0;

pub const CFG_DATALEN_7: u32 = 0 << 2;
pub const CFG_DATALEN_8: u32 = 1 << 2;
pub const CFG_DATALEN_9: u32 = 2 << 2;

pub const CFG_PARITY_NONE: u32 = 0 << 4;
pub const CFG_PARITY_EVEN: u32 = 2 << 4;
pub const CFG_PARITY_ODD: u32 = 3 << 4;

pub const CFG_STOPLEN_1: u32 = 0 << 6;
pub const CFG_STOPLEN_2: u32 = 1 << 6;

pub const CFG_CTSEN: u32 = 1 << 9;

pub const CFG_SYNCEN: u32 = 1 << 11;

pub const CFG_CLKPOL_FALLING: u32 = 0 << 12;
pub const CFG_CLKPOL_RISING: u32 = 1 << 12;

pub const CFG_SYNCMST: u32 = 1 << 14;

pub const CFG_LOOP: u32 = 1 << 15;

#[cfg(not(feature = "lpc81x"))]
pub const CFG_OETA: u32 = 1 << 18;

#[cfg(not(feature = "lpc81x"))]
pub const CFG_AUTOADDR: u32 = 1 << 19;

#[cfg(not(feature = "lpc81x"))]
pub const CFG_OESEL: u32 = 1 << 20;

#[cfg(not(feature = "lpc81x"))]
pub const CFG_OEPOL_LOW: u32 = 0 << 21;
#[cfg(not(feature = "lpc81x"))]
pub const CFG_OEPOL_HIGH: u32 = 1 << 21;

#[cfg(not(feature = "lpc81x"))]
pub const CFG_RXPOL_INVERTED: u32 = 1 << 22;

#[cfg(not(feature = "lpc81x"))]
pub const CFG_TXPOL_INVERTED: u32 = 1 << 23;

pub const CTL_TXBRKEN: u32 = 1 << 1;
pub const CTL_ADDRDET: u32 = 1 << 2;
pub const CTL_TXDIS: u32 = 1 << 6;
pub const CTL_CC: u32 = 1 << 8;
pub const CTL_CLRCONRX: u32 = 1 << 9;

#[cfg(not(feature = "lpc81x"))]
pub const CTL_AUTOBAUD: u32 = 1 << 16;

#[cfg(feature = "lpc81x")]
pub const CTL_RESERVED: u32 = !(CTL_TXBRKEN | CTL_ADDRDET | CTL_TXDIS | CTL_CC | CTL_CLRCONRX);
#[cfg(not(feature = "lpc81x"))]
pub const CTL_RESERVED: u32 =
    !(CTL_TXBRKEN | CTL_ADDRDET | CTL_TXDIS | CTL_CC | CTL_CLRCONRX | CTL_AUTOBAUD);

pub const STAT_RXRDY: u32 = 1 << 0;
pub const STAT_RXIDLE: u32 = 1 << 1;
pub const STAT_TXRDY: u32 = 1 << 2;
pub const STAT_TXIDLE: u32 = 1 << 3;
pub const STAT_CTS: u32 = 1 << 4;
pub const STAT_DELTACTS: u32 = 1 << 5;
pub const STAT_TXDISSTAT: u32 = 1 << 6;
pub const STAT_OVERRUNINT: u32 = 1 << 8;
pub const STAT_RXBRK: u32 = 1 << 10;
pub const STAT_DELTARXBRK: u32 = 1 << 11;
pub const STAT_START: u32 = 1 << 12;
pub const STAT_FRAMERRINT: u32 = 1 << 13;
pub const STAT_PARITYERRINT: u32 = 1 << 14;
pub const STAT_RXNOISEINT: u32 = 1 << 15;
pub const STAT_ABERR: u32 = 1 << 16;

impl Usart {
    pub fn init(&self, divisor: u32) {
        unsafe {
            let config = CFG_DATALEN_8 | CFG_PARITY_NONE | CFG_STOPLEN_1;
            self.brg().write(divisor - 1);
            self.cfg().write(CFG_ENABLE | config);
        }
    }
    pub fn tx_ready(&self) -> bool {
        unsafe { (self.stat().read() & STAT_TXRDY) != 0 }
    }
    pub fn rx_ready(&self) -> bool {
        unsafe { (self.stat().read() & STAT_RXRDY) != 0 }
    }
    pub fn tx(&self, b: u8) {
        while !self.tx_ready() {}
        unsafe {
            self.txdat().write(b as u32);
        }
    }
    pub fn rx(&self) -> u8 {
        while !self.rx_ready() {}
        unsafe { self.rxdat().read() as u8 }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_usart() {
        let usart0 = unsafe { &mut *::USART0 };
        let usart1 = unsafe { &mut *::USART1 };
        let usart2 = unsafe { &mut *::USART2 };

        assert_eq!(address(&usart0.cfg()), 0x4006_4000);
        assert_eq!(address(&usart0.ctl()), 0x4006_4004);
        assert_eq!(address(&usart0.stat()), 0x4006_4008);
        assert_eq!(address(&usart0.intenset()), 0x4006_400c);
        assert_eq!(address(&usart0.intenclr()), 0x4006_4010);
        assert_eq!(address(&usart0.rxdat()), 0x4006_4014);
        assert_eq!(address(&usart0.rxdatstat()), 0x4006_4018);
        assert_eq!(address(&usart0.txdat()), 0x4006_401c);
        assert_eq!(address(&usart0.brg()), 0x4006_4020);
        assert_eq!(address(&usart0.intstat()), 0x4006_4024);
        assert_eq!(address(&usart0.osr()), 0x4006_4028);
        assert_eq!(address(&usart0.addr()), 0x4006_402c);

        assert_eq!(address(&usart1.cfg()), 0x4006_8000);
        assert_eq!(address(&usart2.cfg()), 0x4006_c000);
    }

    fn address<T>(r: *const T) -> usize {
        r as usize
    }
}
