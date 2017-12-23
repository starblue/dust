#[used]
#[link_section = ".irq_vectors"]
pub static IRQ_VECTORS: [Option<fn()>; 32] = [
    Some(pio0_0_irq_handler), //  0
    Some(pio0_1_irq_handler),
    Some(pio0_2_irq_handler),
    Some(pio0_3_irq_handler),
    Some(pio0_4_irq_handler),
    Some(pio0_5_irq_handler), //  5
    Some(pio0_6_irq_handler),
    Some(pio0_7_irq_handler),
    Some(pio0_8_irq_handler),
    Some(pio0_9_irq_handler),
    Some(pio0_10_irq_handler), // 10
    Some(pio0_11_irq_handler),
    Some(pio1_0_irq_handler),
    Some(c_can_irq_handler),
    Some(spi1_irq_handler),
    Some(i2c_irq_handler), // 15
    Some(ct16b0_irq_handler),
    Some(ct16b1_irq_handler),
    Some(ct32b0_irq_handler),
    Some(ct32b1_irq_handler),
    Some(spi0_irq_handler), // 20
    Some(uart_irq_handler),
    None,
    None,
    Some(adc_irq_handler),
    Some(wdt_irq_handler), // 25
    Some(bod_irq_handler),
    None,
    Some(gpio3_irq_handler),
    Some(gpio2_irq_handler),
    Some(gpio1_irq_handler), // 30
    Some(gpio0_irq_handler),
];

#[linkage = "weak"]
pub fn pio0_0_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn pio0_1_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn pio0_2_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn pio0_3_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn pio0_4_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn pio0_5_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn pio0_6_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn pio0_7_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn pio0_8_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn pio0_9_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn pio0_10_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn pio0_11_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn pio1_0_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn c_can_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn spi1_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn i2c_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn ct16b0_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn ct16b1_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn ct32b0_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn ct32b1_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn spi0_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn uart_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn adc_irq_handler() {
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
pub fn gpio3_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn gpio2_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn gpio1_irq_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn gpio0_irq_handler() {
    loop {}
}
