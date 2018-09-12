use dust::default_handler;

default_handler!(pio0_0_irq_handler);
default_handler!(pio0_1_irq_handler);
default_handler!(pio0_2_irq_handler);
default_handler!(pio0_3_irq_handler);
default_handler!(pio0_4_irq_handler);
default_handler!(pio0_5_irq_handler);
default_handler!(pio0_6_irq_handler);
default_handler!(pio0_7_irq_handler);
default_handler!(pio0_8_irq_handler);
default_handler!(pio0_9_irq_handler);
default_handler!(pio0_10_irq_handler);
default_handler!(pio0_11_irq_handler);
default_handler!(pio1_0_irq_handler);
default_handler!(c_can_irq_handler);
default_handler!(spi1_irq_handler);
default_handler!(i2c_irq_handler);
default_handler!(ct16b0_irq_handler);
default_handler!(ct16b1_irq_handler);
default_handler!(ct32b0_irq_handler);
default_handler!(ct32b1_irq_handler);
default_handler!(spi0_irq_handler);
default_handler!(uart_irq_handler);
default_handler!(adc_irq_handler);
default_handler!(wdt_irq_handler);
default_handler!(bod_irq_handler);
default_handler!(gpio3_irq_handler);
default_handler!(gpio2_irq_handler);
default_handler!(gpio1_irq_handler);
default_handler!(gpio0_irq_handler);

#[used]
#[link_section = ".irq_vectors"]
pub static IRQ_VECTORS: [Option<fn()>; 32] = [
    // 0
    Some(pio0_0_irq_handler),
    Some(pio0_1_irq_handler),
    Some(pio0_2_irq_handler),
    Some(pio0_3_irq_handler),
    // 4
    Some(pio0_4_irq_handler),
    Some(pio0_5_irq_handler),
    Some(pio0_6_irq_handler),
    Some(pio0_7_irq_handler),
    // 8
    Some(pio0_8_irq_handler),
    Some(pio0_9_irq_handler),
    Some(pio0_10_irq_handler),
    Some(pio0_11_irq_handler),
    // 12
    Some(pio1_0_irq_handler),
    Some(c_can_irq_handler),
    Some(spi1_irq_handler),
    Some(i2c_irq_handler),
    // 16
    Some(ct16b0_irq_handler),
    Some(ct16b1_irq_handler),
    Some(ct32b0_irq_handler),
    Some(ct32b1_irq_handler),
    // 20
    Some(spi0_irq_handler),
    Some(uart_irq_handler),
    None,
    None,
    // 24
    Some(adc_irq_handler),
    Some(wdt_irq_handler),
    Some(bod_irq_handler),
    None,
    // 28
    Some(gpio3_irq_handler),
    Some(gpio2_irq_handler),
    Some(gpio1_irq_handler),
    Some(gpio0_irq_handler),
    // 32
];
