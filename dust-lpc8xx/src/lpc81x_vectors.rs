use dust::default_handler;

default_handler!(spi0_irq_handler);
default_handler!(spi1_irq_handler);
default_handler!(uart0_irq_handler);
default_handler!(uart1_irq_handler);
default_handler!(uart2_irq_handler);
default_handler!(i2c0_irq_handler);
default_handler!(sct_irq_handler);
default_handler!(mrt_irq_handler);
default_handler!(cmp_irq_handler);
default_handler!(wdt_irq_handler);
default_handler!(bod_irq_handler);
default_handler!(wkt_irq_handler);
default_handler!(pinint0_irq_handler);
default_handler!(pinint1_irq_handler);
default_handler!(pinint2_irq_handler);
default_handler!(pinint3_irq_handler);
default_handler!(pinint4_irq_handler);
default_handler!(pinint5_irq_handler);
default_handler!(pinint6_irq_handler);
default_handler!(pinint7_irq_handler);

#[used]
#[link_section = ".irq_vectors"]
pub static IRQ_VECTORS: [Option<fn()>; 32] = [
    // 0
    Some(spi0_irq_handler),
    Some(spi1_irq_handler),
    None,
    Some(uart0_irq_handler),
    // 4
    Some(uart1_irq_handler),
    Some(uart2_irq_handler),
    None,
    None,
    // 8
    Some(i2c0_irq_handler),
    Some(sct_irq_handler),
    Some(mrt_irq_handler),
    Some(cmp_irq_handler),
    // 12
    Some(wdt_irq_handler),
    Some(bod_irq_handler),
    None,
    Some(wkt_irq_handler),
    // 16
    None,
    None,
    None,
    None,
    // 20
    None,
    None,
    None,
    None,
    // 24
    Some(pinint0_irq_handler),
    Some(pinint1_irq_handler),
    Some(pinint2_irq_handler),
    Some(pinint3_irq_handler),
    // 28
    Some(pinint4_irq_handler),
    Some(pinint5_irq_handler),
    Some(pinint6_irq_handler),
    Some(pinint7_irq_handler),
    // 32
];
