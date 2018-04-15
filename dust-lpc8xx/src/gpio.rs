use dust::gpio::pin::mode;
use dust::gpio::pin::{InputPin, OutputPin};
use dust::gpio::port;
use volatile_register::RW;

#[cfg(feature = "lpc802")]
pub const PINS: usize = 18;
#[cfg(feature = "lpc804")]
pub const PINS: usize = 31;
#[cfg(feature = "lpc81x")]
pub const PINS: usize = 18;
#[cfg(any(feature = "lpc82x", feature = "lpc83x"))]
pub const PINS: usize = 29;
#[cfg(feature = "lpc84x")]
pub const PINS: usize = 54;

const PORTS: usize = (PINS + 31) / 32;

#[cfg(feature = "lpc81x")]
#[repr(C)]
pub struct Gpio {
    pub b: [RW<u8>; PINS],
    reserved0: [u8; 0x1000 - PINS],
    pub w: [RW<u32>; PINS],
    reserved1: [u8; 0x1000 - 4 * PINS],
    pub dir: [RW<u32>; PORTS],
    reserved2: [u8; 0x80 - 4 * PORTS],
    pub mask: [RW<u32>; PORTS],
    reserved3: [u8; 0x80 - 4 * PORTS],
    pub pin: [RW<u32>; PORTS],
    reserved4: [u8; 0x80 - 4 * PORTS],
    pub mpin: [RW<u32>; PORTS],
    reserved5: [u8; 0x80 - 4 * PORTS],
    pub set: [RW<u32>; PORTS],
    reserved6: [u8; 0x80 - 4 * PORTS],
    pub clr: [RW<u32>; PORTS],
    reserved7: [u8; 0x80 - 4 * PORTS],
    pub not: [RW<u32>; PORTS],
}

#[cfg(not(feature = "lpc81x"))]
#[repr(C)]
pub struct Gpio {
    pub b: [RW<u8>; PINS],
    reserved0: [u8; 0x1000 - PINS],
    pub w: [RW<u32>; PINS],
    reserved1: [u8; 0x1000 - 4 * PINS],
    pub dir: [RW<u32>; PORTS],
    reserved2: [u8; 0x80 - 4 * PORTS],
    pub mask: [RW<u32>; PORTS],
    reserved3: [u8; 0x80 - 4 * PORTS],
    pub pin: [RW<u32>; PORTS],
    reserved4: [u8; 0x80 - 4 * PORTS],
    pub mpin: [RW<u32>; PORTS],
    reserved5: [u8; 0x80 - 4 * PORTS],
    pub set: [RW<u32>; PORTS],
    reserved6: [u8; 0x80 - 4 * PORTS],
    pub clr: [RW<u32>; PORTS],
    reserved7: [u8; 0x80 - 4 * PORTS],
    pub not: [RW<u32>; PORTS],
    reserved8: [u8; 0x80 - 4 * PORTS],
    pub dirset: [RW<u32>; PORTS],
    reserved9: [u8; 0x80 - 4 * PORTS],
    pub dirclr: [RW<u32>; PORTS],
    reserved10: [u8; 0x80 - 4 * PORTS],
    pub dirnot: [RW<u32>; PORTS],
}

pub struct Port<'a> {
    gpio: &'a Gpio,
    index: usize,
}

impl<'a> Port<'a> {
    pub unsafe fn new(gpio: &mut Gpio, index: usize) -> Port {
        Port { gpio, index }
    }
}

impl<'a> port::GetValue<u32> for Port<'a> {
    fn get_value(&mut self) -> u32 {
        self.gpio.pin[self.index].read()
    }
}

impl<'a> port::SetValue<u32> for Port<'a> {
    fn set_value(&mut self, value: u32) {
        unsafe { self.gpio.pin[self.index].write(value) }
    }
    fn modify_value<F>(&mut self, f: F)
    where
        F: FnMut(u32) -> u32,
    {
        unsafe {
            self.gpio.pin[self.index].modify(f);
        }
    }
}

impl<'a> port::Set<u32> for Port<'a> {
    fn set_bits(&mut self, bits: u32) {
        unsafe {
            self.gpio.set[self.index].write(bits);
        }
    }
}

impl<'a> port::Clr<u32> for Port<'a> {
    fn clr_bits(&mut self, bits: u32) {
        unsafe {
            self.gpio.clr[self.index].write(bits);
        }
    }
}

impl<'a> port::Toggle<u32> for Port<'a> {
    fn toggle_bits(&mut self, bits: u32) {
        unsafe {
            self.gpio.not[self.index].write(bits);
        }
    }
}

impl<'a> port::DirSetValue<u32> for Port<'a> {
    fn dir_set_value(&mut self, dir: u32) {
        unsafe {
            self.gpio.dir[self.index].write(dir);
        }
    }
    fn dir_modify_value<F>(&mut self, f: F)
    where
        F: FnMut(u32) -> u32,
    {
        unsafe {
            self.gpio.dir[self.index].modify(f);
        }
    }
}

#[cfg(not(feature = "lpc81x"))]
impl<'a> port::DirSet<u32> for Port<'a> {
    fn dir_set(&mut self, dir: u32) {
        unsafe {
            self.gpio.dirset[self.index].write(dir);
        }
    }
}

#[cfg(not(feature = "lpc81x"))]
impl<'a> port::DirClr<u32> for Port<'a> {
    fn dir_clr(&mut self, dir: u32) {
        unsafe {
            self.gpio.dirclr[self.index].write(dir);
        }
    }
}

#[cfg(not(feature = "lpc81x"))]
impl<'a> port::DirToggle<u32> for Port<'a> {
    fn dir_toggle(&mut self, dir: u32) {
        unsafe {
            self.gpio.dirnot[self.index].write(dir);
        }
    }
}

#[cfg(feature = "lpc81x")]
impl Gpio {
    pub fn set_pin_output(&self, pin: usize) {
        let (port, bit) = port_bit(pin);
        unsafe {
            self.dir[port].modify(|w| w | (1 << bit));
        }
    }
    pub fn set_pin_input(&self, pin: usize) {
        let (port, bit) = port_bit(pin);
        unsafe {
            self.dir[port].modify(|w| w & !(1 << bit));
        }
    }
}

#[cfg(not(feature = "lpc81x"))]
impl Gpio {
    pub fn set_pin_output(&self, pin: usize) {
        let (port, bit) = port_bit(pin);
        unsafe {
            self.dirset[port].write(1 << bit);
        }
    }
    pub fn set_pin_input(&self, pin: usize) {
        let (port, bit) = port_bit(pin);
        unsafe {
            self.dirclr[port].write(1 << bit);
        }
    }
}

impl Gpio {
    pub fn get_pin_value(&self, pin: usize) -> bool {
        self.b[pin].read() != 0
    }
    pub fn set_pin(&self, pin: usize) {
        let (port, bit) = port_bit(pin);
        unsafe {
            self.set[port].write(1 << bit);
        }
    }
    pub fn clr_pin(&self, pin: usize) {
        let (port, bit) = port_bit(pin);
        unsafe {
            self.clr[port].write(1 << bit);
        }
    }
    pub fn toggle_pin(&self, pin: usize) {
        let (port, bit) = port_bit(pin);
        unsafe {
            self.not[port].write(1 << bit);
        }
    }
}

fn port_bit(pin: usize) -> (usize, usize) {
    if pin >= PINS {
        panic!();
    }
    let port = pin / 32;
    let bit = pin % 32;
    (port, bit)
}

pub struct Pin<'a, M> {
    mode: M,
    gpio: &'a Gpio,
    index: usize,
}

impl<'a, M> Pin<'a, M> {
    pub fn new(mode: M, gpio: &mut Gpio, index: usize) -> Pin<M> {
        Pin { mode, gpio, index }
    }
}

impl<'a, M> InputPin for Pin<'a, M>
where
    M: mode::Input,
{
    fn get_value(&mut self) -> bool {
        self.gpio.get_pin_value(self.index)
    }
}

impl<'a, M> OutputPin for Pin<'a, M>
where
    M: mode::Output,
{
    fn set(&mut self) {
        self.gpio.set_pin(self.index);
    }
    fn clr(&mut self) {
        self.gpio.clr_pin(self.index);
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_gpio() {
        let gpio = unsafe { &mut *::GPIO };

        assert_eq!(address(&gpio.b), 0xA000_0000);
        assert_eq!(address(&gpio.w), 0xA000_1000);
        assert_eq!(address(&gpio.dir), 0xA000_2000);
        assert_eq!(address(&gpio.mask), 0xA000_2080);
        assert_eq!(address(&gpio.pin), 0xA000_2100);
        assert_eq!(address(&gpio.mpin), 0xA000_2180);
        assert_eq!(address(&gpio.set), 0xA000_2200);
        assert_eq!(address(&gpio.clr), 0xA000_2280);
        assert_eq!(address(&gpio.not), 0xA000_2300);
    }

    #[cfg(any(feature = "lpc82x", feature = "lpc83x", feature = "lpc84x"))]
    #[test]
    fn test_gpio_82x() {
        let gpio = unsafe { &mut *::GPIO };

        assert_eq!(address(&gpio.dirset), 0xA000_2380);
        assert_eq!(address(&gpio.dirclr), 0xA000_2400);
        assert_eq!(address(&gpio.dirnot), 0xA000_2480);
    }

    fn address<T>(r: *const T) -> usize {
        r as usize
    }
}
