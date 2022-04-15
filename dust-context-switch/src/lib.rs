#![feature(naked_functions)]
#![no_std]

/// Import this constant to create an artificial dependency.
///
/// This is needed to pull the pendsv handler into the executable.
#[no_mangle]
pub static PENDSV_HANDLER: unsafe extern "C" fn() = pendsv_handler;

#[cfg(feature = "v6m")]
pub mod context_switch_v6m;
#[cfg(feature = "v6m")]
pub use context_switch_v6m::pendsv_handler;

#[cfg(feature = "v7m")]
pub mod context_switch_v7m;
#[cfg(feature = "v7m")]
pub use context_switch_v7m::pendsv_handler;
