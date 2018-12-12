//! Reset and Clock Control (RCC)

use volatile_register::RW;

#[repr(C)]
pub struct Rcc {
    pub cr: RW<u32>,
    pub cfgr: RW<u32>,
    pub cir: RW<u32>,
    pub apb2rstr: RW<u32>,

    pub apb1rstr: RW<u32>,
    pub ahbenr: RW<u32>,
    pub apb2enr: RW<u32>,
    pub apb1enr: RW<u32>,

    pub bdcr: RW<u32>,
    pub csr: RW<u32>,
    pub ahbrstr: RW<u32>,
    pub cfgr2: RW<u32>,

    pub cfgr3: RW<u32>,
    pub cr2: RW<u32>,
}

pub const CR_HSION: u32 = 1 << 0;
pub const CR_HSIRDY: u32 = 1 << 1;
pub const CR_HSEON: u32 = 1 << 16;
pub const CR_HSERDY: u32 = 1 << 17;
pub const CR_HSEBYP: u32 = 1 << 18;
pub const CR_CSSON: u32 = 1 << 19;
pub const CR_PLLON: u32 = 1 << 24;
pub const CR_PLLRDY: u32 = 1 << 25;

pub const APB2_SYSCFG: u32 = 1 << 0;
pub const APB2_USART6: u32 = 1 << 5;
pub const APB2_ADC: u32 = 1 << 9;
pub const APB2_TIM1: u32 = 1 << 11;
pub const APB2_SPI1: u32 = 1 << 12;
pub const APB2_USART1: u32 = 1 << 14;
pub const APB2_TIM15: u32 = 1 << 16;
pub const APB2_TIM16: u32 = 1 << 17;
pub const APB2_TIM17: u32 = 1 << 18;
pub const APB2_DBGMCU: u32 = 1 << 22;

pub const APB1_TIM3: u32 = 1 << 1;
pub const APB1_TIM6: u32 = 1 << 4;
pub const APB1_TIM7: u32 = 1 << 5;
pub const APB1_TIM14: u32 = 1 << 8;
pub const APB1_WWDG: u32 = 1 << 11;
pub const APB1_SPI2: u32 = 1 << 14;
pub const APB1_USART2: u32 = 1 << 17;
pub const APB1_USART3: u32 = 1 << 18;
pub const APB1_USART4: u32 = 1 << 19;
pub const APB1_USART5: u32 = 1 << 20;
pub const APB1_I2C1: u32 = 1 << 21;
pub const APB1_I2C2: u32 = 1 << 22;
pub const APB1_USB: u32 = 1 << 23;
pub const APB1_PWR: u32 = 1 << 28;

pub const AHB_DMA: u32 = 1 << 0;
pub const AHB_SRAM: u32 = 1 << 2;
pub const AHB_FLITF: u32 = 1 << 4;
pub const AHB_CRC: u32 = 1 << 6;
pub const AHB_IOPA: u32 = 1 << 17;
pub const AHB_IOPB: u32 = 1 << 18;
pub const AHB_IOPC: u32 = 1 << 19;
pub const AHB_IOPD: u32 = 1 << 20;
pub const AHB_IOPF: u32 = 1 << 22;
