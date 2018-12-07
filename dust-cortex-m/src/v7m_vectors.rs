//! Exception vector table for the v7-M architecture (Cortex-M3, Cortex-M4, Cortex-M7)

use crate::reset_handler;
use dust::default_handler;

default_handler!(nmi_handler);
default_handler!(hard_fault_handler);
default_handler!(memory_fault_handler);
default_handler!(bus_fault_handler);
default_handler!(usage_fault_handler);
default_handler!(svcall_handler);
default_handler!(debug_monitor_handler);
default_handler!(pendsv_handler);
default_handler!(systick_handler);

#[used]
#[link_section = ".exception_vectors"]
pub static VECTORS: [Option<unsafe extern "C" fn()>; 15] = [
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
    Some(debug_monitor_handler),
    None,
    Some(pendsv_handler),
    Some(systick_handler),
];
