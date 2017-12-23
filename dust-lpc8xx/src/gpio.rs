use volatile_register::RW;

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

#[cfg(any(feature = "lpc82x", feature = "lpc83x", feature = "lpc84x"))]
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
