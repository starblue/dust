#[linkage = "weak"]
pub fn spi0_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn spi1_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn uart0_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn uart1_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn uart2_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn i2c1_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn i2c0_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn sct_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn mrt_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn cmp_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn wdt_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn bod_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn flash_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn wkt_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn adc_seqa_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn adc_seqb_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn adc_thcmp_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn adc_ovr_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn dma_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn i2c2_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn i2c3_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn pinint0_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn pinint1_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn pinint2_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn pinint3_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn pinint4_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn pinint5_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn pinint6_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn pinint7_irq_handler() {
    loop {}
}

#[used]
#[link_section = ".irq_vectors"]
pub static IRQ_VECTORS: [Option<fn()>; 32] = [
    Some(spi0_irq_handler),
    Some(spi1_irq_handler),
    None,
    Some(uart0_irq_handler),
    Some(uart1_irq_handler),
    Some(uart2_irq_handler),
    None,
    None,
    Some(i2c0_irq_handler),
    Some(sct_irq_handler),
    Some(mrt_irq_handler),
    Some(cmp_irq_handler),
    Some(wdt_irq_handler),
    Some(bod_irq_handler),
    None,
    Some(wkt_irq_handler),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(pinint0_irq_handler),
    Some(pinint1_irq_handler),
    Some(pinint2_irq_handler),
    Some(pinint3_irq_handler),
    Some(pinint4_irq_handler),
    Some(pinint5_irq_handler),
    Some(pinint6_irq_handler),
    Some(pinint7_irq_handler),
];
