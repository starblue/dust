#![no_std]
#![feature(const_fn)]

///! Volatile hardware registers accessed only via pointers.
///!
///! The pointer is derived from an explicit address,
///! to avoid ever having a reference to the register.
///!
///! A reference would allow the compiler to add spurious reads,
///! though so far that doesn't seem to happen, I'm not aware of a case
///! where the compiler has been caught doing this completely unprovoked.
///! TODO Link to relevant discussions
///! TODO document usage
use core::marker::PhantomData;
use core::ptr::read_volatile;
use core::ptr::write_volatile;

/// Types of registers that can be read.
pub trait Read<T: Copy> {
    /// Performs a volatile read of the register.
    unsafe fn read(&self) -> T;
}

/// Types of registers that can be written.
pub trait Write<T: Copy> {
    /// Performs a volatile write of the register.
    unsafe fn write(&self, value: T);
}

/// Types of registers that can be modified.
pub trait Modify<T: Copy> {
    /// Performs a volatile read-modify-write of the register.
    unsafe fn modify<F>(&self, f: F)
    where
        F: FnOnce(T) -> T;
}

/// A register that is readable and writable.
pub struct RW<T: Copy> {
    address: usize,
    t: PhantomData<T>,
}
impl<T> RW<T>
where
    T: Copy,
{
    /// Create a readable and writeable register at the given address.
    pub const fn at(address: usize) -> RW<T> {
        RW {
            address,
            t: PhantomData,
        }
    }
}
impl<T> RW<T>
where
    T: Copy,
{
    fn ptr(&self) -> *mut T {
        self.address as *mut T
    }
}

impl<T> Read<T> for RW<T>
where
    T: Copy,
{
    /// Performs a volatile read of the register.
    unsafe fn read(&self) -> T {
        read_volatile(self.ptr())
    }
}

impl<T> Write<T> for RW<T>
where
    T: Copy,
{
    /// Performs a volatile write of the register.
    unsafe fn write(&self, value: T) {
        write_volatile(self.ptr(), value);
    }
}

/// A register that is read-only.
pub struct RO<T: Copy>(RW<T>);

impl<T> RO<T>
where
    T: Copy,
{
    /// Create a read-only register at the given address.
    pub const fn at(address: usize) -> RO<T> {
        RO(RW::at(address))
    }
}
impl<T> Read<T> for RO<T>
where
    T: Copy,
{
    unsafe fn read(&self) -> T {
        self.0.read()
    }
}

/// A register that is write-only.
pub struct WO<T: Copy>(RW<T>);
impl<T> WO<T>
where
    T: Copy,
{
    /// Create a write-only register at the given address.
    pub const fn at(address: usize) -> WO<T> {
        WO(RW::at(address))
    }
}
impl<T> Write<T> for WO<T>
where
    T: Copy,
{
    unsafe fn write(&self, value: T) {
        self.0.write(value);
    }
}

impl<R, T> Modify<T> for R
where
    R: Read<T> + Write<T>,
    T: Copy,
{
    unsafe fn modify<F>(&self, f: F)
    where
        F: FnOnce(T) -> T,
    {
        self.write(f(self.read()));
    }
}