//! Switch matrix (SWM)

#![allow(unused_macros)]
#![allow(dead_code)]

#[cfg(feature = "lpc802")]
pub use self::swm_lpc802 as swm_variant;
#[cfg(feature = "lpc804")]
pub use self::swm_lpc804 as swm_variant;
#[cfg(feature = "lpc81x")]
pub use self::swm_lpc81x as swm_variant;
#[cfg(feature = "lpc82x")]
pub use self::swm_lpc82x as swm_variant;
#[cfg(feature = "lpc83x")]
pub use self::swm_lpc83x as swm_variant;
#[cfg(feature = "lpc84x")]
pub use self::swm_lpc84x as swm_variant;

pub use self::swm_variant::*;
use volatile_register::RW;

#[repr(C)]
pub struct Swm {
    pub pinassign: [RW<u32>; PINASSIGN_REGISTERS],
    reserved0: [u8; 0x180 - 4 * PINASSIGN_REGISTERS],
    pub pinassign_fixed: [RW<u32>; PINASSIGN_FIXED_REGISTERS],
    reserved1: [u8; 0x1c0 - 4 * PINASSIGN_FIXED_REGISTERS - 0x180],
    pub pinenable: [RW<u32>; PINENABLE_REGISTERS],
}

#[derive(Clone, Copy, Debug)]
pub struct MovableFunction {
    reg: usize,
    index: usize,
}

macro_rules! movable_function {
    ($name:ident, $reg:expr, $index:expr) => {
        pub const $name: MovableFunction = MovableFunction {
            reg: $reg,
            index: $index,
        };
    };
}

#[derive(Clone, Copy, Debug)]
pub struct MovableFixedFunction {
    reg: usize,
    index: usize,
    pin0: usize,
    pin1: usize,
    pin2: usize,
}

macro_rules! movable_fixed_function {
    ($name:ident, $reg:expr, $index:expr => $pin0:expr, $pin1:expr, $pin2:expr) => {
        pub const $name: MovableFixedFunction = MovableFixedFunction {
            reg: $reg,
            index: $index,
            pin0: $pin0,
            pin1: $pin1,
            pin2: $pin2,
        };
    };
}

#[derive(Clone, Copy, Debug)]
pub struct FixedFunction {
    reg: usize,
    index: usize,
}

macro_rules! fixed_pin_function {
    ($name:ident, $reg:expr, $index:expr) => {
        pub const $name: FixedFunction = FixedFunction {
            reg: $reg,
            index: $index,
        };
    };
}

#[cfg(feature = "lpc802")]
pub mod swm_lpc802;
#[cfg(feature = "lpc804")]
pub mod swm_lpc804;
#[cfg(feature = "lpc81x")]
pub mod swm_lpc81x;
#[cfg(feature = "lpc82x")]
pub mod swm_lpc82x;
#[cfg(feature = "lpc83x")]
pub mod swm_lpc83x;
#[cfg(feature = "lpc84x")]
pub mod swm_lpc84x;

impl Swm {
    pub unsafe fn disable_movable_function(&mut self, f: MovableFunction) {
        self.set_movable_function_pin_internal(f, 0xff);
    }
    pub unsafe fn set_movable_function_pin(&self, f: MovableFunction, pin: usize) {
        self.set_movable_function_pin_internal(f, pin);
    }
    unsafe fn set_movable_function_pin_internal(&self, f: MovableFunction, pin: usize) {
        let shift = 8 * f.index;
        let mask: u32 = 0xff << shift;
        let data: u32 = (pin as u32) << shift;
        self.pinassign[f.reg].modify(|w| (w & !mask) | data);
    }

    pub unsafe fn disable_movable_fixed_function(&mut self, f: MovableFixedFunction) {
        self.set_movable_fixed_function_pin_internal(f, 3);
    }
    pub unsafe fn set_movable_fixed_function_pin(&self, f: MovableFixedFunction, pin: usize) {
        if pin == f.pin0 {
            self.set_movable_fixed_function_pin_internal(f, 0);
        } else if pin == f.pin1 {
            self.set_movable_fixed_function_pin_internal(f, 1);
        } else if pin == f.pin2 {
            self.set_movable_fixed_function_pin_internal(f, 2);
        } else {
            panic!("movable fixed function cannot be assigned to requested pin");
        }
    }
    unsafe fn set_movable_fixed_function_pin_internal(
        &self,
        f: MovableFixedFunction,
        value: usize,
    ) {
        let shift = 2 * f.index;
        let mask: u32 = 3 << shift;
        let data: u32 = (value as u32) << shift;
        self.pinassign[f.reg].modify(|w| (w & !mask) | data);
    }

    pub unsafe fn enable_fixed_function(&mut self, f: FixedFunction) {
        self.pinenable[f.reg].modify(|b| b & !(1 << f.index));
    }
    pub unsafe fn disable_fixed_function(&mut self, f: FixedFunction) {
        self.pinenable[f.reg].modify(|b| b | (1 << f.index));
    }
}
