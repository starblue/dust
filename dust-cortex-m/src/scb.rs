use volatile_register::{RO, RW};

/// System Control Block
#[cfg(feature = "thumbv6m")]
#[repr(C)]
pub struct Scb {
    /// CPUID base register
    cpuid: RO<u32>,
    /// Interrupt Control and State Register
    icsr: RW<u32>,
    /// Vector Table Offset Register
    vtor: RW<u32>,
    /// Application Interrupt and Reset Control Register
    aircr: RW<u32>,
    /// System Control Register
    scr: RW<u32>,
    /// Configuration and Control Register
    ccr: RW<u32>,
    /// System Handler Priority Register
    shpr: [RW<u32>; 3],
    /// System Handler Control and State Register
    shcsr: RW<u32>,
    reserved_d28: [u32; 2],
    /// Debug Fault Status Register
    dfsr: RW<u32>,
}
#[cfg(feature = "thumbv7m")]
#[repr(C)]
pub struct Scb {
    /// CPUID base register
    cpuid: RO<u32>,
    /// Interrupt Control and State Register
    icsr: RW<u32>,
    /// Vector Table Offset Register
    vtor: RW<u32>,
    /// Application Interrupt and Reset Control Register
    aircr: RW<u32>,
    /// System Control Register
    scr: RW<u32>,
    /// Configuration and Control Register
    ccr: RW<u32>,
    /// System Handler Priority Register
    shpr: [RW<u32>; 3],
    /// System Handler Control and State Register
    shcsr: RW<u32>,
    /// Configurable Fault Status Register
    cfsr: RW<u32>,
    /// HardFault Status Register
    hfsr: RW<u32>,
    /// Debug Fault Status Register
    dfsr: RW<u32>,
    /// MemManage Fault Address Register
    mmfar: RW<u32>,
    /// Bus Fault Address Register
    bfar: RW<u32>,
    /// Auxiliary Fault Status Register
    afsr: RW<u32>,
    // CPUID registers
    /// Processor Feature Register 0..1
    id_pfr0: RO<u32>,
    /// Processor Feature Register 0..1
    id_pfr1: RO<u32>,
    /// Debug Feature Register 0
    id_dfr0: RO<u32>,
    /// Auxiliary Feature Register 0
    id_afr0: RO<u32>,
    /// Memory Model Feature Register 0
    id_mmfr0: RO<u32>,
    /// Memory Model Feature Register 1
    id_mmfr1: RO<u32>,
    /// Memory Model Feature Register 2
    id_mmfr2: RO<u32>,
    /// Memory Model Feature Register 3
    id_mmfr3: RO<u32>,
    /// Instruction Set Attribute Feature Register 0
    id_isar0: RO<u32>,
    /// Instruction Set Attribute Feature Register 1
    id_isar1: RO<u32>,
    /// Instruction Set Attribute Feature Register 2
    id_isar2: RO<u32>,
    /// Instruction Set Attribute Feature Register 3
    id_isar3: RO<u32>,
    /// Instruction Set Attribute Feature Register 4
    id_isar4: RO<u32>,
    /// Instruction Set Attribute Feature Register 5
    id_isar5: RO<u32>,
    reserved_d78: [u32; 2],
    reserved_d80: [u32; 2],
    /// Coprocessor Access Control Register
    cpacr: RW<u32>,
}

#[cfg(test)]
mod test {
    #[cfg(any(feature = "thumbv6m", feature = "thumbv7m"))]
    #[test]
    fn test_scb() {
        let scb = unsafe { &mut *::SCB };

        assert_eq!(address(&scb.cpuid), 0xE000_ED00);
        assert_eq!(address(&scb.icsr), 0xE000_ED04);
        assert_eq!(address(&scb.vtor), 0xE000_ED08);
        assert_eq!(address(&scb.aircr), 0xE000_ED0C);
        assert_eq!(address(&scb.scr), 0xE000_ED10);
        assert_eq!(address(&scb.ccr), 0xE000_ED14);
        assert_eq!(address(&scb.shpr), 0xE000_ED18);
        assert_eq!(address(&scb.shcsr), 0xE000_ED24);
        assert_eq!(address(&scb.dfsr), 0xE000_ED30);
    }

    #[cfg(feature = "thumbv7m")]
    #[test]
    fn test_scb_v7m() {
        let scb = unsafe { &mut *::SCB };

        assert_eq!(address(&scb.cfsr), 0xE000_ED28);
        assert_eq!(address(&scb.hfsr), 0xE000_ED2C);
        assert_eq!(address(&scb.mmfar), 0xE000_ED34);
        assert_eq!(address(&scb.bfar), 0xE000_ED38);
        assert_eq!(address(&scb.afsr), 0xE000_ED3C);
        assert_eq!(address(&scb.id_pfr0), 0xE000_ED40);
        assert_eq!(address(&scb.id_pfr1), 0xE000_ED44);
        assert_eq!(address(&scb.id_dfr0), 0xE000_ED48);
        assert_eq!(address(&scb.id_afr0), 0xE000_ED4C);
        assert_eq!(address(&scb.id_mmfr0), 0xE000_ED50);
        assert_eq!(address(&scb.id_mmfr1), 0xE000_ED54);
        assert_eq!(address(&scb.id_mmfr2), 0xE000_ED58);
        assert_eq!(address(&scb.id_mmfr3), 0xE000_ED5C);
        assert_eq!(address(&scb.id_isar0), 0xE000_ED60);
        assert_eq!(address(&scb.id_isar1), 0xE000_ED64);
        assert_eq!(address(&scb.id_isar2), 0xE000_ED68);
        assert_eq!(address(&scb.id_isar3), 0xE000_ED6C);
        assert_eq!(address(&scb.id_isar4), 0xE000_ED70);
        assert_eq!(address(&scb.id_isar5), 0xE000_ED74);
        assert_eq!(address(&scb.cpacr), 0xE000_ED88);
    }

    fn address<T>(r: *const T) -> usize {
        r as usize
    }
}
