/// SYSCON peripheral unit for LPC802
///
/// Checked against UM11045 2018-03-26.
use volatile_register::{RO, RW};

#[repr(C)]
pub struct Syscon {
    pub sysmemremap: RW<u32>,
    reserved_0x004: u32,
    reserved_0x008: u32,
    reserved_0x00c: u32,
    reserved_0x010: u32,
    reserved_0x014: u32,
    reserved_0x018: u32,
    reserved_0x01c: u32,
    reserved_0x020: u32,
    reserved_0x024: u32,
    reserved_0x028: u32,
    reserved_0x02c: u32,
    reserved_0x030: u32,
    reserved_0x034: u32,
    pub sysrststat: RW<u32>,
    reserved_0x03c: u32,
    reserved_0x040: u32,
    reserved_0x044: u32,
    reserved_0x048: u32,
    reserved_0x04c: u32,
    pub mainclksel: RW<u32>,
    pub mainclkuen: RW<u32>,
    pub sysahbclkdiv: RW<u32>,
    reserved_0x05c: u32,
    reserved_0x060: u32,
    pub adcclksel: RW<u32>,
    pub adcclkdiv: RW<u32>,
    reserved_0x06c: u32,
    reserved_0x070: u32,
    reserved_0x074: u32,
    reserved_0x078: u32,
    pub lposcclken: RW<u32>,
    pub sysahbclkctrl: [RW<u32>; 1],
    reserved_0x084: u32,
    pub presetctrl: [RW<u32>; 2],
    pub uart0clksel: RW<u32>,
    pub uart1clksel: RW<u32>,
    reserved_0x098: u32,
    reserved_0x09c: u32,
    reserved_0x0a0: u32,
    pub i2c0clksel: RW<u32>,
    reserved_0x0a8: u32,
    reserved_0x0ac: u32,
    reserved_0x0b0: u32,
    pub spi0clksel: RW<u32>,
    reserved_0x0b8: u32,
    reserved_0x0bc: u32,
    reserved_0x0c0: u32,
    reserved_0x0c4: u32,
    reserved_0x0c8: u32,
    reserved_0x0cc: u32,
    pub frg0div: RW<u32>,
    pub frg0mult: RW<u32>,
    pub frg0clksel: RW<u32>,
    reserved_0x0dc: u32,
    reserved_0x0e0: u32,
    reserved_0x0e4: u32,
    reserved_0x0e8: u32,
    reserved_0x0ec: u32,
    pub clkoutsel: RW<u32>,
    pub clkoutdiv: RW<u32>,
    reserved_0x0f8: u32,
    reserved_0x0fc: u32,
    pub pioporcap: [RO<u32>; 1],
    reserved_0x104: [u8; 0x0134 - 0x104],
    reserved_0x134: u32,
    reserved_0x138: u32,
    reserved_0x13c: u32,
    reserved_0x140: u32,
    reserved_0x144: u32,
    reserved_0x148: u32,
    reserved_0x14c: u32,
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
pub const RESET_FLASH: Reset = Reset(0, 4);
pub const RESET_I2C0: Reset = Reset(0, 5);
pub const RESET_GPIO0: Reset = Reset(0, 6);
pub const RESET_SWM: Reset = Reset(0, 7);
pub const RESET_WKT: Reset = Reset(0, 9);
pub const RESET_MRT: Reset = Reset(0, 10);
pub const RESET_SPI0: Reset = Reset(0, 11);
pub const RESET_CRC: Reset = Reset(0, 13);
pub const RESET_UART0: Reset = Reset(0, 14);
pub const RESET_UART1: Reset = Reset(0, 15);
pub const RESET_IOCON: Reset = Reset(0, 18);
pub const RESET_ACMP: Reset = Reset(0, 19);
pub const RESET_ADC: Reset = Reset(0, 24);
pub const RESET_CTIMER0: Reset = Reset(0, 25);
pub const RESET_GPIOINT: Reset = Reset(0, 28);

pub const RESET_FRG0: Reset = Reset(1, 3);

pub const PRESETCTRL_RESERVED: [u32; 2] = [0xe4d3110f, 0xffffffd6];

#[derive(Clone, Copy, Debug)]
pub struct Clock(usize, u32);

pub const CLOCK_SYS: Clock = Clock(0, 0);
pub const CLOCK_ROM: Clock = Clock(0, 1);
pub const CLOCK_RAM0: Clock = Clock(0, 2);
pub const CLOCK_FLASH: Clock = Clock(0, 4);
pub const CLOCK_I2C0: Clock = Clock(0, 5);
pub const CLOCK_GPIO0: Clock = Clock(0, 6);
pub const CLOCK_SWM: Clock = Clock(0, 7);
pub const CLOCK_WKT: Clock = Clock(0, 9);
pub const CLOCK_MRT: Clock = Clock(0, 10);
pub const CLOCK_SPI0: Clock = Clock(0, 11);
pub const CLOCK_CRC: Clock = Clock(0, 13);
pub const CLOCK_UART0: Clock = Clock(0, 14);
pub const CLOCK_UART1: Clock = Clock(0, 15);
pub const CLOCK_WWDT: Clock = Clock(0, 17);
pub const CLOCK_IOCON: Clock = Clock(0, 18);
pub const CLOCK_ACMP: Clock = Clock(0, 19);
pub const CLOCK_ADC: Clock = Clock(0, 24);
pub const CLOCK_CTIMER0: Clock = Clock(0, 25);
pub const CLOCK_GPIO_INT: Clock = Clock(0, 28);

pub const SYSAHBCLKCTRL_RESERVED: [u32; 2] = [0xe4d11108, 0xffffffde];

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
        assert_eq!(address(&syscon.sysrststat), 0x4004_8038);
        assert_eq!(address(&syscon.mainclksel), 0x4004_8050);
        assert_eq!(address(&syscon.mainclkuen), 0x4004_8054);
        assert_eq!(address(&syscon.sysahbclkdiv), 0x4004_8058);
        assert_eq!(address(&syscon.adcclksel), 0x4004_8064);
        assert_eq!(address(&syscon.adcclkdiv), 0x4004_8068);
        assert_eq!(address(&syscon.lposcclken), 0x4004_807c);
        assert_eq!(address(&syscon.sysahbclkctrl), 0x4004_8080);
        assert_eq!(address(&syscon.presetctrl), 0x4004_8088);
        assert_eq!(address(&syscon.uart0clksel), 0x4004_8090);
        assert_eq!(address(&syscon.uart1clksel), 0x4004_8094);
        assert_eq!(address(&syscon.i2c0clksel), 0x4004_80a4);
        assert_eq!(address(&syscon.frg0div), 0x4004_80d0);
        assert_eq!(address(&syscon.frg0mult), 0x4004_80d4);
        assert_eq!(address(&syscon.frg0clksel), 0x4004_80d8);
        assert_eq!(address(&syscon.clkoutsel), 0x4004_80f0);
        assert_eq!(address(&syscon.clkoutdiv), 0x4004_80f4);
        assert_eq!(address(&syscon.pioporcap), 0x4004_8100);
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
