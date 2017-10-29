use volatile_register::{RO, RW, WO};

#[repr(C)]
pub struct Syscon {
    pub sysmemremap: RW<u32>,
    pub presetctrl: RW<u32>,
    pub syspllctrl: RW<u32>,
    pub syspllstat: RO<u32>,
    reserved_0x010: [u8; 0x020 - 0x010],
    pub sysoscctrl: RW<u32>,
    pub wdtoscctrl: RW<u32>,
    pub ircctrl: RW<u32>,
    reserved_0x02c: [u8; 0x030 - 0x02c],
    pub sysrststat: RW<u32>,
    reserved_0x034: [u8; 0x040 - 0x034],
    pub syspllclksel: RW<u32>,
    pub syspllclkuen: RW<u32>,
    reserved_0x048: [u8; 0x070 - 0x048],
    pub mainclksel: RW<u32>,
    pub mainclkuen: RW<u32>,
    pub sysahbclkdiv: RW<u32>,
    reserved_0x07c: [u8; 0x080 - 0x07c],
    pub sysahbclkctrl: RW<u32>,
    reserved_0x084: [u8; 0x094 - 0x084],
    pub ssp0clkdiv: RW<u32>,
    pub uartclkdiv: RW<u32>,
    pub ssp1clkdiv: RW<u32>,
    reserved_0x0a0: [u8; 0x0d0 - 0x0a0],
    pub wdtclksel: RW<u32>,
    pub wdtclkuen: RW<u32>,
    pub wdtclkdiv: RW<u32>,
    reserved_0x0dc: [u8; 0x0e0 - 0x0dc],
    pub clkoutsel: RW<u32>,
    pub clkoutuen: RW<u32>,
    pub clkoutdiv: RW<u32>,
    reserved_0x0ec: [u8; 0x100 - 0x0ec],
    pub pioporcap0: RW<u32>,
    pub pioporcap1: RW<u32>,
    reserved_0x108: [u8; 0x0150 - 0x108],
    pub bodctrl: RW<u32>,
    pub systckcal: RW<u32>,
    reserved_0x158: [u8; 0x170 - 0x158],
    pub irqlatency: RW<u32>,
    pub nmisrc: RW<u32>,
    reserved_0x178: [u8; 0x200 - 0x178],
    pub startaprp0: RW<u32>,
    pub starterp0: RW<u32>,
    pub startrsrp0clr: WO<u32>,
    pub startsrp0: RO<u32>,
    reserved_0x210: [u8; 0x230 - 0x210],
    pub pdsleepcfg: RW<u32>,
    pub pdawakecfg: RW<u32>,
    pub pdruncfg: RW<u32>,
    reserved_0x23c: [u8; 0x3f4 - 0x23c],
    pub device_id: RO<u32>,
}


pub const SYSMEMREMAP_BOOT_ROM: u32 = 0 << 0;
pub const SYSMEMREMAP_RAM: u32 = 1 << 0;
pub const SYSMEMREMAP_FLASH: u32 = 2 << 0;


#[derive(Clone, Copy, Debug)]
pub struct Reset(u32);

pub const RESET_SSP0: Reset = Reset(0);
pub const RESET_I2C: Reset = Reset(1);
pub const RESET_SSP1: Reset = Reset(2);
pub const RESET_CAN: Reset = Reset(3);

pub const PRESETCTRL_RESERVED: u32 = 0xfffffff0;



#[derive(Clone, Copy, Debug)]
pub struct Clock(u32);

pub const CLOCK_SYS: Clock = Clock(0);
pub const CLOCK_ROM: Clock = Clock(1);
pub const CLOCK_RAM0_1: Clock = Clock(2);
pub const CLOCK_FLASHREG: Clock = Clock(3);
pub const CLOCK_FLASH: Clock = Clock(4);
pub const CLOCK_I2C: Clock = Clock(5);
pub const CLOCK_GPIO: Clock = Clock(6);
pub const CLOCK_CT16B0: Clock = Clock(7);
pub const CLOCK_CT16B1: Clock = Clock(8);
pub const CLOCK_CT32B0: Clock = Clock(9);
pub const CLOCK_CT32B1: Clock = Clock(10);
pub const CLOCK_SSP0: Clock = Clock(11);
pub const CLOCK_UART: Clock = Clock(12);
pub const CLOCK_ADC: Clock = Clock(13);
pub const CLOCK_WDT: Clock = Clock(15);
pub const CLOCK_IOCON: Clock = Clock(16);
pub const CLOCK_CAN: Clock = Clock(17);
pub const CLOCK_SSP1: Clock = Clock(18);

pub const SYSAHBCLKCTRL_RESERVED: u32 = 0xfff84000;


impl Syscon {
    pub unsafe fn assert_periph_reset(&self, reset: Reset) {
        self.presetctrl
            .modify(|w| !(1 << reset.0) & (w & !PRESETCTRL_RESERVED));
    }
    pub unsafe fn deassert_periph_reset(&self, reset: Reset) {
        self.presetctrl
            .modify(|w| (1 << reset.0) | (w & !PRESETCTRL_RESERVED));
    }
    pub unsafe fn periph_reset(&self, reset: Reset) {
        self.assert_periph_reset(reset);
        self.deassert_periph_reset(reset);
    }

    pub unsafe fn enable_clock(&self, clock: Clock) {
        self.sysahbclkctrl
            .modify(|w| (1 << clock.0) | (w & !SYSAHBCLKCTRL_RESERVED));
    }
    pub unsafe fn disable_clock(&self, clock: Clock) {
        self.sysahbclkctrl
            .modify(|w| !(1 << clock.0) & (w & !SYSAHBCLKCTRL_RESERVED));
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
        assert_eq!(address(&syscon.ssp0clkdiv), 0x4004_8094);
        assert_eq!(address(&syscon.uartclkdiv), 0x4004_8098);
        assert_eq!(address(&syscon.ssp1clkdiv), 0x4004_809c);
        assert_eq!(address(&syscon.wdtclksel), 0x4004_80d0);
        assert_eq!(address(&syscon.wdtclkuen), 0x4004_80d4);
        assert_eq!(address(&syscon.wdtclkdiv), 0x4004_80d8);
        assert_eq!(address(&syscon.clkoutsel), 0x4004_80e0);
        assert_eq!(address(&syscon.clkoutuen), 0x4004_80e4);
        assert_eq!(address(&syscon.clkoutdiv), 0x4004_80e8);
        assert_eq!(address(&syscon.pioporcap0), 0x4004_8100);
        assert_eq!(address(&syscon.pioporcap1), 0x4004_8104);
        assert_eq!(address(&syscon.bodctrl), 0x4004_8150);
        assert_eq!(address(&syscon.systckcal), 0x4004_8154);
        assert_eq!(address(&syscon.irqlatency), 0x4004_8170);
        assert_eq!(address(&syscon.nmisrc), 0x4004_8174);
        assert_eq!(address(&syscon.startaprp0), 0x4004_8200);
        assert_eq!(address(&syscon.starterp0), 0x4004_8204);
        assert_eq!(address(&syscon.startrsrp0clr), 0x4004_8208);
        assert_eq!(address(&syscon.startsrp0), 0x4004_820c);
        assert_eq!(address(&syscon.pdsleepcfg), 0x4004_8230);
        assert_eq!(address(&syscon.pdawakecfg), 0x4004_8234);
        assert_eq!(address(&syscon.pdruncfg), 0x4004_8238);
        assert_eq!(address(&syscon.device_id), 0x4004_83F4);
    }

    fn address<T>(r: *const T) -> usize {
        r as usize
    }
}
