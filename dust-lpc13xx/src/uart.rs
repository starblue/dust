use volatile_register::{RO, RW};

#[repr(C)]
pub struct Uart {
    pub rbr_thr_dll: RW<u32>,
    pub ier_dlm: RW<u32>,
    pub iir_fcr: RW<u32>,
    pub lcr: RW<u32>,
    pub mcr: RW<u32>,
    pub lsr: RO<u32>,
    pub msr: RO<u32>,
    pub scr: RW<u32>,
    pub acr: RW<u32>,
    reserved_024: u32,
    pub fdr: RW<u32>,
    reserved_02c: u32,
    pub ter: RW<u32>,
    reserved_034: [u8; 0x04c - 0x034],
    pub rs485ctrl: RW<u32>,
    pub rs485adrmatch: RW<u32>,
    pub rs485dly: RW<u32>,
}

pub const IER_RBRIE: u32 = 1 << 0;
pub const IER_THREIE: u32 = 1 << 1;
pub const IER_RXLIE: u32 = 1 << 2;
pub const IER_ABEOINTEN: u32 = 1 << 8;
pub const IER_ABTOINTEN: u32 = 1 << 9;
pub const IER_RESERVED_WZ: u32 = 0xfffffc70;

pub const IIR_INTSTATUS: u32 = 1 << 0;
pub const IIR_ABEOINT: u32 = 1 << 8;
pub const IIR_ABTOINT: u32 = 1 << 9;
pub const IIR_RESERVED_WZ: u32 = 0xfffffc30;

pub const FCR_FIFOEN: u32 = 1 << 0;

pub const FCR_RXFIFORES: u32 = 1 << 1;

pub const FCR_TXFIFORES: u32 = 1 << 2;

pub const FCR_RXTL_1: u32 = 0 << 6;
pub const FCR_RXTL_4: u32 = 1 << 6;
pub const FCR_RXTL_8: u32 = 2 << 6;
pub const FCR_RXTL_14: u32 = 3 << 6;

pub const FCR_RESERVED_WZ: u32 = 0xffffff38;

pub const LCR_WLS_5: u32 = 0 << 0;
pub const LCR_WLS_6: u32 = 1 << 0;
pub const LCR_WLS_7: u32 = 2 << 0;
pub const LCR_WLS_8: u32 = 3 << 0;

pub const LCR_SBS_1: u32 = 0 << 2;
pub const LCR_SBS_2: u32 = 1 << 2;

pub const LCR_PE: u32 = 1 << 3;

pub const LCR_PS_ODD: u32 = 0 << 4;
pub const LCR_PS_EVEN: u32 = 1 << 4;
pub const LCR_PS_FORCED_1: u32 = 2 << 4;
pub const LCR_PS_FORCED_0: u32 = 3 << 4;

pub const LCR_BC: u32 = 1 << 6;

pub const LCR_DLAB: u32 = 1 << 7;

pub const MCR_DTRC: u32 = 1 << 0;
pub const MCR_RTSC: u32 = 1 << 1;
pub const MCR_LMS: u32 = 1 << 4;
pub const MCR_RTSEN: u32 = 1 << 6;
pub const MCR_CTSEN: u32 = 1 << 7;
pub const MCR_RESERVED_WZ: u32 = 0xffffff2c;

pub const LSR_RDR: u32 = 1 << 0;
pub const LSR_OE: u32 = 1 << 1;
pub const LSR_PE: u32 = 1 << 2;
pub const LSR_FE: u32 = 1 << 3;
pub const LSR_BI: u32 = 1 << 4;
pub const LSR_THRE: u32 = 1 << 5;
pub const LSR_TEMT: u32 = 1 << 6;
pub const LSR_RXFE: u32 = 1 << 7;

pub const MCR_DCTS: u32 = 1 << 0;
pub const MCR_DDSR: u32 = 1 << 1;
pub const MCR_TERI: u32 = 1 << 2;
pub const MCR_DDCD: u32 = 1 << 3;
pub const MCR_CTS: u32 = 1 << 4;
pub const MCR_DSR: u32 = 1 << 5;
pub const MCR_RI: u32 = 1 << 6;
pub const MCR_DCD: u32 = 1 << 7;

pub const ACR_START: u32 = 1 << 0;
pub const ACR_MODE: u32 = 1 << 1;
pub const ACR_AUTORESTART: u32 = 1 << 2;
pub const ACR_ABEOINTCLR: u32 = 1 << 8;
pub const ACR_ABTOINTCLR: u32 = 1 << 9;
pub const ACR_RESERVED_WZ: u32 = 0xfffffcf8;

pub const TER_TXEN: u32 = 1 << 7;
pub const TER_RESERVED_WZ: u32 = !TER_TXEN;

impl Uart {
    pub fn init(&self) {
        unsafe {
            // Assumes 12 MHz PCLK, sets 115200 8N1
            // 12000000 / (16 * 4 * (1 + 5 / 8))
            // 12000000 / (64 * 13 / 8)
            // 12000000 / (8 * 13)
            // 1500000 / 13 = 115384
            let divaddval = 5;
            let mulval = 8;
            self.fdr.write(divaddval | (mulval << 4));

            self.lcr.write(LCR_DLAB);
            self.rbr_thr_dll.write(4);
            self.ier_dlm.write(0);

            self.lcr.write(LCR_WLS_8 | LCR_SBS_1);

            self.iir_fcr.write(FCR_FIFOEN);
        }
    }
    pub fn tx_ready(&self) -> bool {
        (self.lsr.read() & LSR_THRE) != 0
    }
    pub fn rx_ready(&self) -> bool {
        (self.lsr.read() & LSR_RDR) != 0
    }
    pub fn tx(&self, b: u8) {
        while !self.tx_ready() {}
        unsafe {
            self.rbr_thr_dll.write(b as u32);
        }
    }
    pub fn rx(&self) -> u8 {
        while !self.rx_ready() {}
        self.rbr_thr_dll.read() as u8
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_uart() {
        let uart = unsafe { &mut *::UART };

        assert_eq!(address(&uart.rbr_thr_dll), 0x4000_8000);
        assert_eq!(address(&uart.ier_dlm), 0x4000_8004);
        assert_eq!(address(&uart.iir_fcr), 0x4000_8008);
        assert_eq!(address(&uart.lcr), 0x4000_800c);
        assert_eq!(address(&uart.mcr), 0x4000_8010);
        assert_eq!(address(&uart.lsr), 0x4000_8014);
        assert_eq!(address(&uart.msr), 0x4000_8018);
        assert_eq!(address(&uart.scr), 0x4000_801c);
        assert_eq!(address(&uart.acr), 0x4000_8020);
        assert_eq!(address(&uart.fdr), 0x4000_8028);
        assert_eq!(address(&uart.ter), 0x4000_8030);
        assert_eq!(address(&uart.rs485ctrl), 0x4000_804c);
        assert_eq!(address(&uart.rs485adrmatch), 0x4000_8050);
        assert_eq!(address(&uart.rs485dly), 0x4000_8054);
    }

    fn address<T>(r: *const T) -> usize {
        r as usize
    }
}
