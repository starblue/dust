default_handler!(pm_irq_handler);
default_handler!(sysctrl_irq_handler);
default_handler!(wdt_irq_handler);
default_handler!(rtc_irq_handler);
default_handler!(eic_irq_handler);
default_handler!(nvmctrl_irq_handler);
default_handler!(dmac_irq_handler);
default_handler!(evsys_irq_handler);
default_handler!(sercom0_irq_handler);
default_handler!(sercom1_irq_handler);
default_handler!(tc1_irq_handler);
default_handler!(tc2_irq_handler);
default_handler!(adc_irq_handler);

#[used]
#[link_section = ".irq_vectors"]
pub static IRQ_VECTORS: [Option<fn()>; 32] = [
    // 0
    Some(pm_irq_handler),
    Some(sysctrl_irq_handler),
    Some(wdt_irq_handler),
    Some(rtc_irq_handler),
    // 4
    Some(eic_irq_handler),
    Some(nvmctrl_irq_handler),
    Some(dmac_irq_handler),
    None,
    // 8
    Some(evsys_irq_handler),
    Some(sercom0_irq_handler),
    Some(sercom1_irq_handler),
    None,
    // 12
    None,
    Some(tc1_irq_handler),
    Some(tc2_irq_handler),
    Some(adc_irq_handler),
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
    None,
    None,
    None,
    None,
    // 28
    None,
    None,
    None,
    None,
    // 32
];
