/// Traits that a GPIO port may implement
pub mod port {
    /// Gets the input value.
    pub trait GetValue<T> {
        fn get_value(&mut self) -> T;
    }

    /// Sets the output value.
    pub trait SetValue<T> {
        fn set_value(&mut self, value: T);
        fn modify_value<F>(&mut self, f: F)
        where
            F: FnMut(T) -> T;
    }

    /// Sets the output value on a subset of the port bits.
    pub trait SetValueMasked<T> {
        fn set_value_masked(&mut self, value: T, mask: T);
    }

    /// Sets an arbitrary subset of the ports bits
    pub trait Set<T> {
        fn set_bits(&mut self, bits: T);
    }

    /// Clears an arbitrary subset of the ports bits.
    pub trait Clr<T> {
        fn clr_bits(&mut self, bits: T);
    }

    /// Toggles an arbitrary subset of the ports bits.
    pub trait Toggle<T> {
        fn toggle_bits(&mut self, bits: T);
    }

    /// Sets or clears an arbitrary subset of the ports bits.
    pub trait SetClr<T> {
        fn set_clr_bits(&mut self, set_bits: T, clr_bits: T);
    }

    /// Sets the direction of the port bits.
    ///
    /// The bit value 1 indicates that the direction is output,
    /// 0 indicates input.
    ///
    pub trait DirSetValue<T> {
        fn dir_set_value(&mut self, dir: T);
        fn dir_modify_value<F>(&mut self, f: F)
        where
            F: FnMut(T) -> T;
    }

    /// Sets some direction bits of the port to output.
    pub trait DirSet<T> {
        fn dir_set(&mut self, dir: T);
    }

    /// Sets some direction bits of the port to input.
    pub trait DirClr<T> {
        fn dir_clr(&mut self, dir: T);
    }

    /// Toggles some direction bits of the port.
    pub trait DirToggle<T> {
        fn dir_toggle(&mut self, dir: T);
    }
}

pub mod pin {

    pub mod mode {
        pub struct I {}
        pub struct O {}
        pub struct IO {}

        pub trait Input {}
        pub trait Output {}

        impl Input for I {}
        impl Output for O {}
        impl Input for IO {}
        impl Output for IO {}
    }

    pub trait InputPin {
        fn get_value(&mut self) -> bool;
    }

    pub trait OutputPin {
        fn set_value(&mut self, v: bool) {
            if v {
                self.set();
            } else {
                self.clr();
            }
        }
        fn set(&mut self);
        fn clr(&mut self);
    }

    pub trait TogglePin {
        fn toggle(&mut self);
    }
}
