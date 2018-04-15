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
default_handler!(pio1_1_irq_handler);
default_handler!(pio1_2_irq_handler);
default_handler!(pio1_3_irq_handler);
default_handler!(pio1_4_irq_handler);
default_handler!(pio1_5_irq_handler);
default_handler!(pio1_6_irq_handler);
default_handler!(pio1_7_irq_handler);
default_handler!(pio1_8_irq_handler);
default_handler!(pio1_9_irq_handler);
default_handler!(pio1_10_irq_handler);
default_handler!(pio1_11_irq_handler);
default_handler!(pio2_0_irq_handler);
default_handler!(pio2_1_irq_handler);
default_handler!(pio2_2_irq_handler);
default_handler!(pio2_3_irq_handler);
default_handler!(pio2_4_irq_handler);
default_handler!(pio2_5_irq_handler);
default_handler!(pio2_6_irq_handler);
default_handler!(pio2_7_irq_handler);
default_handler!(pio2_8_irq_handler);
default_handler!(pio2_9_irq_handler);
default_handler!(pio2_10_irq_handler);
default_handler!(pio2_11_irq_handler);
default_handler!(pio3_0_irq_handler);
default_handler!(pio3_1_irq_handler);
default_handler!(pio3_2_irq_handler);
default_handler!(pio3_3_irq_handler);
default_handler!(i2c0_irq_handler);
default_handler!(ct16b0_irq_handler);
default_handler!(ct16b1_irq_handler);
default_handler!(ct32b0_irq_handler);
default_handler!(ct32b1_irq_handler);
default_handler!(ssp0_irq_handler);
default_handler!(uart_irq_handler);
default_handler!(usb_irq_handler);
default_handler!(usb_fiq_handler);
default_handler!(adc_irq_handler);
default_handler!(wdt_irq_handler);
default_handler!(bod_irq_handler);
default_handler!(gpio3_irq_handler);
default_handler!(gpio2_irq_handler);
default_handler!(gpio1_irq_handler);
default_handler!(gpio0_irq_handler);
default_handler!(ssp1_irq_handler);


#[used]
#[link_section = ".irq_vectors"]
pub static IRQ_VECTORS: [Option<fn()>; 64] = [
    //  0
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
    // 12,
    Some(pio1_0_irq_handler),
    Some(pio1_1_irq_handler),
    Some(pio1_2_irq_handler),
    Some(pio1_3_irq_handler),
    // 16
    Some(pio1_4_irq_handler),
    Some(pio1_5_irq_handler),
    Some(pio1_6_irq_handler),
    Some(pio1_7_irq_handler),
    // 20
    Some(pio1_8_irq_handler),
    Some(pio1_9_irq_handler),
    Some(pio1_10_irq_handler),
    Some(pio1_11_irq_handler),
    // 24,
    Some(pio2_0_irq_handler),
    Some(pio2_1_irq_handler),
    Some(pio2_2_irq_handler),
    Some(pio2_3_irq_handler),
    // 28
    Some(pio2_4_irq_handler),
    Some(pio2_5_irq_handler),
    Some(pio2_6_irq_handler),
    Some(pio2_7_irq_handler),
    // 32
    Some(pio2_8_irq_handler),
    Some(pio2_9_irq_handler),
    Some(pio2_10_irq_handler),
    Some(pio2_11_irq_handler),
    // 36,
    Some(pio3_0_irq_handler),
    Some(pio3_1_irq_handler),
    Some(pio3_2_irq_handler),
    Some(pio3_3_irq_handler),
    // 40
    Some(i2c0_irq_handler),  
    Some(ct16b0_irq_handler),
    Some(ct16b1_irq_handler),
    Some(ct32b0_irq_handler),
    // 44
    Some(ct32b1_irq_handler),
    Some(ssp0_irq_handler),
    Some(uart_irq_handler),
    Some(usb_irq_handler),
    // 48
    Some(usb_fiq_handler),
    Some(adc_irq_handler),
    Some(wdt_irq_handler),
    Some(bod_irq_handler),
    // 52
    None,
    Some(gpio3_irq_handler),
    Some(gpio2_irq_handler),
    Some(gpio1_irq_handler),
    // 56
    Some(gpio0_irq_handler),
    Some(ssp1_irq_handler),
    None,
    None,
    // 60
    None,
    None,
    None,
    None,
    // 64
];
