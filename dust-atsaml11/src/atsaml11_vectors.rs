default_handler!(pm_irq_handler);
default_handler!(wdt_irq_handler);
default_handler!(rtc_irq_handler);
default_handler!(eic_0_irq_handler);
default_handler!(eic_1_irq_handler);
default_handler!(eic_2_irq_handler);
default_handler!(eic_3_irq_handler);
default_handler!(eic_other_irq_handler);
default_handler!(freqm_irq_handler);
default_handler!(nvmctrl_irq_handler);
default_handler!(port_irq_handler);
default_handler!(dmac_0_irq_handler);
default_handler!(dmac_1_irq_handler);
default_handler!(dmac_2_irq_handler);
default_handler!(dmac_3_irq_handler);
default_handler!(dmac_other_irq_handler);
default_handler!(evsys_0_irq_handler);
default_handler!(evsys_1_irq_handler);
default_handler!(evsys_2_irq_handler);
default_handler!(evsys_3_irq_handler);
default_handler!(evsys_nschk_irq_handler);
default_handler!(pac_irq_handler);
default_handler!(sercom0_0_irq_handler);
default_handler!(sercom0_1_irq_handler);
default_handler!(sercom0_2_irq_handler);
default_handler!(sercom0_other_irq_handler);
default_handler!(sercom1_0_irq_handler);
default_handler!(sercom1_1_irq_handler);
default_handler!(sercom1_2_irq_handler);
default_handler!(sercom1_other_irq_handler);
default_handler!(sercom2_0_irq_handler);
default_handler!(sercom2_1_irq_handler);
default_handler!(sercom2_2_irq_handler);
default_handler!(sercom2_other_irq_handler);
default_handler!(tc0_irq_handler);
default_handler!(tc1_irq_handler);
default_handler!(tc2_irq_handler);
default_handler!(adc_other_irq_handler);
default_handler!(adc_resrdy_irq_handler);
default_handler!(ac_irq_handler);
default_handler!(dac_underrun_a_irq_handler);
default_handler!(dac_empty_irq_handler);
default_handler!(ptc_empty_irq_handler);
default_handler!(trng_irq_handler);
default_handler!(tram_irq_handler);

#[used]
#[link_section = ".irq_vectors"]
pub static IRQ_VECTORS: [Option<fn()>; 45] = [
    // 0
    Some(pm_irq_handler),
    Some(wdt_irq_handler),
    Some(rtc_irq_handler),
    Some(eic_0_irq_handler),
    // 4
    Some(eic_1_irq_handler),
    Some(eic_2_irq_handler),
    Some(eic_3_irq_handler),
    Some(eic_other_irq_handler),
    // 8
    Some(freqm_irq_handler),
    Some(nvmctrl_irq_handler),
    Some(port_irq_handler),
    Some(dmac_0_irq_handler),
    // 12
    Some(dmac_1_irq_handler),
    Some(dmac_2_irq_handler),
    Some(dmac_3_irq_handler),
    Some(dmac_other_irq_handler),
    // 16
    Some(evsys_0_irq_handler),
    Some(evsys_1_irq_handler),
    Some(evsys_2_irq_handler),
    Some(evsys_3_irq_handler),
    // 20
    Some(evsys_nschk_irq_handler),
    Some(pac_irq_handler),
    Some(sercom0_0_irq_handler),
    Some(sercom0_1_irq_handler),
    // 24
    Some(sercom0_2_irq_handler),
    Some(sercom0_other_irq_handler),
    Some(sercom1_0_irq_handler),
    Some(sercom1_1_irq_handler),
    // 28
    Some(sercom1_2_irq_handler),
    Some(sercom1_other_irq_handler),
    Some(sercom2_0_irq_handler),
    Some(sercom2_1_irq_handler),
    // 32
    Some(sercom2_2_irq_handler),
    Some(sercom2_other_irq_handler),
    Some(tc0_irq_handler),
    Some(tc1_irq_handler),
    // 36
    Some(tc2_irq_handler),
    Some(adc_other_irq_handler),
    Some(adc_resrdy_irq_handler),
    Some(ac_irq_handler),
    // 40
    Some(dac_underrun_a_irq_handler),
    Some(dac_empty_irq_handler),
    Some(ptc_empty_irq_handler),
    Some(trng_irq_handler),
    // 44
    Some(tram_irq_handler),
    // 45
];
