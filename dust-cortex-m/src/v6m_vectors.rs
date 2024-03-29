//! Exception vector table for the v6-M architecture (Cortex-M0, Cortex-M0+)

use crate::reset_handler;
use dust::default_handler;

default_handler!(nmi_handler);
default_handler!(hard_fault_handler);
default_handler!(svcall_handler);
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
