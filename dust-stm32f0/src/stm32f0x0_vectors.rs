use dust::default_handler;

default_handler!(wwdt_irq_handler);
default_handler!(rtc_irq_handler);
default_handler!(flash_irq_handler);
default_handler!(rcc_irq_handler);
default_handler!(exti0_1_irq_handler);
default_handler!(exti2_3_irq_handler);
default_handler!(exti4_15_irq_handler);
default_handler!(dma_ch1_irq_handler);
default_handler!(dma_ch2_3_irq_handler);
default_handler!(dma_ch4_5_irq_handler);
default_handler!(adc_irq_handler);
default_handler!(tim1_brk_up_trg_com_irq_handler);
default_handler!(tim1_cc_irq_handler);
default_handler!(tim3_irq_handler);
default_handler!(tim6_irq_handler);
default_handler!(tim14_irq_handler);
default_handler!(tim15_irq_handler);
default_handler!(tim16_irq_handler);
default_handler!(tim17_irq_handler);
default_handler!(i2c1_irq_handler);
default_handler!(i2c2_irq_handler);
default_handler!(spi1_irq_handler);
default_handler!(spi2_irq_handler);
default_handler!(usart1_irq_handler);
default_handler!(usart2_irq_handler);
default_handler!(usart3_4_5_6_irq_handler);
default_handler!(usb_irq_handler);

#[used]
#[link_section = ".irq_vectors"]
pub static IRQ_VECTORS: [Option<unsafe extern "C" fn()>; 32] = [
    // 0
    Some(wwdt_irq_handler),
    None,
    Some(rtc_irq_handler),
    Some(flash_irq_handler),
    // 4
    Some(rcc_irq_handler),
    Some(exti0_1_irq_handler),
    Some(exti2_3_irq_handler),
    Some(exti4_15_irq_handler),
    // 8
    None,
    Some(dma_ch1_irq_handler),
    Some(dma_ch2_3_irq_handler),
    Some(dma_ch4_5_irq_handler),
    // 12
    Some(adc_irq_handler),
    Some(tim1_brk_up_trg_com_irq_handler),
    Some(tim1_cc_irq_handler),
    None,
    // 16
    Some(tim3_irq_handler),
    Some(tim6_irq_handler),
    None,
    Some(tim14_irq_handler),
    // 20
    Some(tim15_irq_handler),
    Some(tim16_irq_handler),
    Some(tim17_irq_handler),
    Some(i2c1_irq_handler),
    // 24
    Some(i2c2_irq_handler),
    Some(spi1_irq_handler),
    Some(spi2_irq_handler),
    Some(usart1_irq_handler),
    // 28
    Some(usart2_irq_handler),
    Some(usart3_4_5_6_irq_handler),
    None,
    Some(usb_irq_handler),
    // 32
];
