#![feature(asm)]
#![feature(naked_functions)]
#![no_std]

extern "C" {
    /// Implement this function for storing and retrieving the context.
    ///
    /// Must return the psp for the context to switch to.
    ///
    /// # Arguments
    ///
    /// * `previous_psp` - The psp of the context to leave.
    #[no_mangle]
    fn dust_switch_context(previous_psp: u32) -> u32;
}

/// Import this constant to create an artificial dependency.
///
/// This is needed to pull the pendsv handler into the executable.
#[no_mangle]
pub static PENDSV_HANDLER: unsafe extern "C" fn() = pendsv_handler;

#[cfg(feature = "v6m")]
pub mod context_switch_v6m;
#[cfg(feature = "v7m")]
pub mod context_switch_v7m;

#[cfg(feature = "v6m")]
pub use context_switch_v6m::pendsv_handler;
#[cfg(feature = "v7m")]
pub use context_switch_v7m::pendsv_handler;
