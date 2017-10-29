use reset_handler;

#[used]
#[link_section = ".cm3_vectors"]
pub static VECTORS: [Option<fn()>; 15] = [
    Some(reset_handler),
    Some(nmi_handler),
    Some(hard_fault_handler),
    Some(memory_fault_handler),
    Some(bus_fault_handler),
    Some(usage_fault_handler),
    None,
    None,
    None,
    None,
    Some(svcall_handler),
    None,
    None,
    Some(pendsv_handler),
    Some(systick_handler),
];

#[linkage = "weak"]
pub fn nmi_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn hard_fault_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn memory_fault_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn bus_fault_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn usage_fault_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn svcall_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn pendsv_handler() {
    loop {}
}

#[linkage = "weak"]
pub fn systick_handler() {
    loop {}
}
