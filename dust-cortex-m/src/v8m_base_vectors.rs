//! Exception vector table for the v8-M baseline architecture (Cortex-M23)

use crate::reset_handler;
use dust::default_handler;

default_handler!(nmi_handler);
default_handler!(hard_fault_handler);
default_handler!(secure_fault_handler);
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
    None,
    None,
    None,
    Some(secure_fault_handler),
    None,
    None,
    None,
    Some(svcall_handler),
    Some(debug_monitor_handler),
    None,
    Some(pendsv_handler),
    Some(systick_handler),
];
