/// SYSCON peripheral unit for LPC82x
///
/// Checked against UM11065 2016-10-05.
use volatile_register::{RO, RW};

#[repr(C)]
pub struct Syscon {
    pub sysmemremap: RW<u32>,
    pub presetctrl: [RW<u32>; 1],
    pub syspllctrl: RW<u32>,
    pub syspllstat: RO<u32>,
    reserved_0x010: u32,
    reserved_0x014: u32,
    reserved_0x018: u32,
    reserved_0x01c: u32,
    pub sysoscctrl: RW<u32>,
    pub wdtoscctrl: RW<u32>,
    pub ircctrl: RW<u32>,
    reserved_0x02c: u32,
    pub sysrststat: RW<u32>,
    reserved_0x034: u32,
    reserved_0x038: u32,
    reserved_0x03c: u32,
    pub syspllclksel: RW<u32>,
    pub syspllclkuen: RW<u32>,
    reserved_0x048: u32,
    reserved_0x04c: u32,
    reserved_0x050: u32,
    reserved_0x054: u32,
    reserved_0x058: u32,
    reserved_0x05c: u32,
    reserved_0x060: u32,
    reserved_0x064: u32,
    reserved_0x068: u32,
    reserved_0x06c: u32,
    pub mainclksel: RW<u32>,
    pub mainclkuen: RW<u32>,
    pub sysahbclkdiv: RW<u32>,
    reserved_0x07c: u32,
    pub sysahbclkctrl: [RW<u32>; 1],
    reserved_0x084: u32,
    reserved_0x088: u32,
    reserved_0x08c: u32,
    reserved_0x090: u32,
    pub uartclkdiv: RW<u32>,
    reserved_0x098: u32,
    reserved_0x09c: u32,
    reserved_0x0a0: u32,
    reserved_0x0a4: u32,
    reserved_0x0a8: u32,
    reserved_0x0ac: u32,
    reserved_0x0b0: u32,
    reserved_0x0b4: u32,
    reserved_0x0b8: u32,
    reserved_0x0bc: u32,
    reserved_0x0c0: u32,
    reserved_0x0c4: u32,
    reserved_0x0c8: u32,
    reserved_0x0cc: u32,
    reserved_0x0d0: u32,
    reserved_0x0d4: u32,
    reserved_0x0d8: u32,
    reserved_0x0dc: u32,
    pub clkoutsel: RW<u32>,
    pub clkoutuen: RW<u32>,
    pub clkoutdiv: RW<u32>,
    reserved_0x0ec: u32,
    pub uartfrgdiv: RW<u32>,
    pub uartfrgmult: RW<u32>,
    reserved_0x0f8: u32,
    pub exttracecmd: RW<u32>,
    pub pioporcap: [RO<u32>; 1],
    reserved_0x104: [u8; 0x0134 - 0x104],
    pub ioconclkdiv6: RW<u32>,
    pub ioconclkdiv5: RW<u32>,
    pub ioconclkdiv4: RW<u32>,
    pub ioconclkdiv3: RW<u32>,
    pub ioconclkdiv2: RW<u32>,
    pub ioconclkdiv1: RW<u32>,
    pub ioconclkdiv0: RW<u32>,
    pub bodctrl: RW<u32>,
    pub systckcal: RW<u32>,
    reserved_0x158: [u8; 0x170 - 0x158],
    pub irqlatency: RW<u32>,
    pub nmisrc: RW<u32>,
    pub pintsel: [RW<u32>; 8],
    reserved_0x198: [u8; 0x204 - 0x198],
    pub starterp0: RW<u32>,
    reserved_0x208: [u8; 0x214 - 0x208],
    pub starterp1: RW<u32>,
    reserved_0x218: [u8; 0x230 - 0x218],
    pub pdsleepcfg: RW<u32>,
    pub pdawakecfg: RW<u32>,
    pub pdruncfg: RW<u32>,
    reserved_0x23c: [u8; 0x3f8 - 0x23c],
    pub device_id: RO<u32>,
}

pub const SYSMEMREMAP_BOOT_ROM: u32 = 0 << 0;
pub const SYSMEMREMAP_RAM: u32 = 1 << 0;
pub const SYSMEMREMAP_FLASH: u32 = 2 << 0;

#[derive(Clone, Copy, Debug)]
pub struct Reset(usize, u32);

// Reset bits in PRESETCTRL
pub const RESET_SPI0: Reset = Reset(0, 0);
pub const RESET_SPI1: Reset = Reset(0, 1);
pub const RESET_UARTFRG: Reset = Reset(0, 2);
pub const RESET_USART0: Reset = Reset(0, 3);
pub const RESET_USART1: Reset = Reset(0, 4);
pub const RESET_USART2: Reset = Reset(0, 5);
pub const RESET_I2C0: Reset = Reset(0, 6);
pub const RESET_MRT: Reset = Reset(0, 7);
pub const RESET_SCT: Reset = Reset(0, 8);
pub const RESET_WKT: Reset = Reset(0, 9);
pub const RESET_GPIO: Reset = Reset(0, 10);
pub const RESET_FLASH: Reset = Reset(0, 11);
pub const RESET_ACMP: Reset = Reset(0, 12);
pub const RESET_I2C1: Reset = Reset(0, 14);
pub const RESET_I2C2: Reset = Reset(0, 15);
pub const RESET_I2C3: Reset = Reset(0, 16);
pub const RESET_ADC: Reset = Reset(0, 24);
pub const RESET_DMA: Reset = Reset(0, 29);

pub const PRESETCTRL_RESERVED: [u32; 1] = [0xdefe2000];

#[derive(Clone, Copy, Debug)]
pub struct Clock(usize, u32);

// Clock bits in SYSAHBCLKCTRL
pub const CLOCK_SYS: Clock = Clock(0, 0);
pub const CLOCK_ROM: Clock = Clock(0, 1);
pub const CLOCK_RAM: Clock = Clock(0, 2);
pub const CLOCK_FLASHREG: Clock = Clock(0, 3);
pub const CLOCK_FLASH: Clock = Clock(0, 4);
pub const CLOCK_I2C0: Clock = Clock(0, 5);
pub const CLOCK_GPIO: Clock = Clock(0, 6);
pub const CLOCK_SWM: Clock = Clock(0, 7);
pub const CLOCK_SCT: Clock = Clock(0, 8);
pub const CLOCK_WKT: Clock = Clock(0, 9);
pub const CLOCK_MRT: Clock = Clock(0, 10);
pub const CLOCK_SPI0: Clock = Clock(0, 11);
pub const CLOCK_SPI1: Clock = Clock(0, 12);
pub const CLOCK_CRC: Clock = Clock(0, 13);
pub const CLOCK_UART0: Clock = Clock(0, 14);
pub const CLOCK_UART1: Clock = Clock(0, 15);
pub const CLOCK_UART2: Clock = Clock(0, 16);
pub const CLOCK_WWDT: Clock = Clock(0, 17);
pub const CLOCK_IOCON: Clock = Clock(0, 18);
pub const CLOCK_ACMP: Clock = Clock(0, 19);
pub const CLOCK_I2C1: Clock = Clock(0, 21);
pub const CLOCK_I2C2: Clock = Clock(0, 22);
pub const CLOCK_I2C3: Clock = Clock(0, 23);
pub const CLOCK_ADC: Clock = Clock(0, 24);
pub const CLOCK_MTB: Clock = Clock(0, 26);
pub const CLOCK_DMA: Clock = Clock(0, 29);

pub const SYSAHBCLKCTRL_RESERVED: [u32; 1] = [0xda100000];

impl Syscon {
    pub unsafe fn assert_periph_reset(&self, reset: Reset) {
        self.presetctrl[reset.0].modify(|w| !(1 << reset.1) & (w & !PRESETCTRL_RESERVED[reset.0]));
    }
    pub unsafe fn deassert_periph_reset(&self, reset: Reset) {
        self.presetctrl[reset.0].modify(|w| (1 << reset.1) | (w & !PRESETCTRL_RESERVED[reset.0]));
    }
    pub unsafe fn periph_reset(&self, reset: Reset) {
        self.assert_periph_reset(reset);
        self.deassert_periph_reset(reset);
    }

    pub unsafe fn enable_clock(&self, clock: Clock) {
        self.sysahbclkctrl[clock.0]
            .modify(|w| (1 << clock.1) | (w & !SYSAHBCLKCTRL_RESERVED[clock.0]));
    }
    pub unsafe fn disable_clock(&self, clock: Clock) {
        self.sysahbclkctrl[clock.0]
            .modify(|w| !(1 << clock.1) & (w & !SYSAHBCLKCTRL_RESERVED[clock.0]));
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_syscon() {
        let syscon = unsafe { &mut *::SYSCON };

        assert_eq!(address(&syscon.sysmemremap), 0x4004_8000);
        assert_eq!(address(&syscon.presetctrl), 0x4004_8004);
        assert_eq!(address(&syscon.syspllctrl), 0x4004_8008);
        assert_eq!(address(&syscon.syspllstat), 0x4004_800c);
        assert_eq!(address(&syscon.sysoscctrl), 0x4004_8020);
        assert_eq!(address(&syscon.wdtoscctrl), 0x4004_8024);
        assert_eq!(address(&syscon.ircctrl), 0x4004_8028);
        assert_eq!(address(&syscon.sysrststat), 0x4004_8030);
        assert_eq!(address(&syscon.syspllclksel), 0x4004_8040);
        assert_eq!(address(&syscon.syspllclkuen), 0x4004_8044);
        assert_eq!(address(&syscon.mainclksel), 0x4004_8070);
        assert_eq!(address(&syscon.mainclkuen), 0x4004_8074);
        assert_eq!(address(&syscon.sysahbclkdiv), 0x4004_8078);
        assert_eq!(address(&syscon.sysahbclkctrl), 0x4004_8080);
        assert_eq!(address(&syscon.uartclkdiv), 0x4004_8094);
        assert_eq!(address(&syscon.clkoutsel), 0x4004_80e0);
        assert_eq!(address(&syscon.clkoutuen), 0x4004_80e4);
        assert_eq!(address(&syscon.clkoutdiv), 0x4004_80e8);
        assert_eq!(address(&syscon.uartfrgdiv), 0x4004_80f0);
        assert_eq!(address(&syscon.uartfrgmult), 0x4004_80f4);
        assert_eq!(address(&syscon.exttracecmd), 0x4004_80fc);
        assert_eq!(address(&syscon.pioporcap), 0x4004_8100);
        assert_eq!(address(&syscon.ioconclkdiv6), 0x4004_8134);
        assert_eq!(address(&syscon.ioconclkdiv5), 0x4004_8138);
        assert_eq!(address(&syscon.ioconclkdiv4), 0x4004_813c);
        assert_eq!(address(&syscon.ioconclkdiv3), 0x4004_8140);
        assert_eq!(address(&syscon.ioconclkdiv2), 0x4004_8144);
        assert_eq!(address(&syscon.ioconclkdiv1), 0x4004_8148);
        assert_eq!(address(&syscon.ioconclkdiv0), 0x4004_814c);
        assert_eq!(address(&syscon.bodctrl), 0x4004_8150);
        assert_eq!(address(&syscon.systckcal), 0x4004_8154);
        assert_eq!(address(&syscon.irqlatency), 0x4004_8170);
        assert_eq!(address(&syscon.nmisrc), 0x4004_8174);
        assert_eq!(address(&syscon.pintsel[0]), 0x4004_8178);
        assert_eq!(address(&syscon.pintsel[7]), 0x4004_8194);
        assert_eq!(address(&syscon.starterp0), 0x4004_8204);
        assert_eq!(address(&syscon.starterp1), 0x4004_8214);
        assert_eq!(address(&syscon.pdsleepcfg), 0x4004_8230);
        assert_eq!(address(&syscon.pdawakecfg), 0x4004_8234);
        assert_eq!(address(&syscon.pdruncfg), 0x4004_8238);
        assert_eq!(address(&syscon.device_id), 0x4004_83F8);
    }

    fn address<T>(r: *const T) -> usize {
        r as usize
    }
}
