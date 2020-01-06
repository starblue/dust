use volatile_register::{RO, RW};

/// System Control Block
///
/// This is the v8-M mainline version with all registers.
/// Smaller cores implement only a subset of registers,
/// and some registers depend on configuration options,
/// but this is currently not implemented.
// TODO implement different configurations
#[repr(C)]
pub struct Scb {
    /// CPUID base register
    pub cpuid: RO<u32>,
    /// Interrupt Control and State Register
    pub icsr: RW<u32>,
    /// Vector Table Offset Register
    pub vtor: RW<u32>,
    /// Application Interrupt and Reset Control Register
    pub aircr: RW<u32>,
    /// System Control Register
    pub scr: RW<u32>,
    /// Configuration and Control Register
    pub ccr: RW<u32>,
    /// System Handler Priority Register
    pub shpr: [RW<u32>; 3],
    /// System Handler Control and State Register
    pub shcsr: RW<u32>,
    /// Configurable Fault Status Register
    pub cfsr: RW<u32>,
    /// HardFault Status Register
    pub hfsr: RW<u32>,
    /// Debug Fault Status Register
    pub dfsr: RW<u32>,
    /// MemManage Fault Address Register
    pub mmfar: RW<u32>,
    /// Bus Fault Address Register
    pub bfar: RW<u32>,
    /// Auxiliary Fault Status Register
    pub afsr: RW<u32>,
    /// Processor Feature Register 0..1
    pub id_pfr0: RO<u32>,
    /// Processor Feature Register 0..1
    pub id_pfr1: RO<u32>,
    /// Debug Feature Register 0
    pub id_dfr0: RO<u32>,
    /// Auxiliary Feature Register 0
    pub id_afr0: RO<u32>,
    /// Memory Model Feature Register 0
    pub id_mmfr0: RO<u32>,
    /// Memory Model Feature Register 1
    pub id_mmfr1: RO<u32>,
    /// Memory Model Feature Register 2
    pub id_mmfr2: RO<u32>,
    /// Memory Model Feature Register 3
    pub id_mmfr3: RO<u32>,
    /// Instruction Set Attribute Feature Register 0
    pub id_isar0: RO<u32>,
    /// Instruction Set Attribute Feature Register 1
    pub id_isar1: RO<u32>,
    /// Instruction Set Attribute Feature Register 2
    pub id_isar2: RO<u32>,
    /// Instruction Set Attribute Feature Register 3
    pub id_isar3: RO<u32>,
    /// Instruction Set Attribute Feature Register 4
    pub id_isar4: RO<u32>,
    /// Instruction Set Attribute Feature Register 5
    pub id_isar5: RO<u32>,
    /// Cache Level ID Register
    pub clidr: RO<u32>,
    /// Cache Type Register
    pub ctr: RO<u32>,
    /// Current Cache Size ID Register
    pub ccsidr: RO<u32>,
    /// Cache Size Selection Register
    pub csselr: RW<u32>,
    /// Coprocessor Access Control Register
    pub cpacr: RW<u32>,
    /// Non-secure Access Control Register
    pub nsacr: RW<u32>,
}

pub const SCB_CCR_NONBASETHRDENA: u32 = 1 << 0;
pub const SCB_CCR_USERSETMPEND: u32 = 1 << 1;
pub const SCB_CCR_UNALIGN_TRP: u32 = 1 << 3;
pub const SCB_CCR_DIV_0_TRP: u32 = 1 << 4;
pub const SCB_CCR_BFHFNMIGN: u32 = 1 << 8;
pub const SCB_CCR_STKALIGN: u32 = 1 << 9;
pub const SCB_CCR_DC: u32 = 1 << 16;
pub const SCB_CCR_IC: u32 = 1 << 17;
pub const SCB_CCR_BP: u32 = 1 << 18;


#[cfg(test)]
mod test {
    use crate::SCB;

    #[test]
    fn test_scb() {
        let scb = unsafe { &mut *SCB };

        assert_eq!(address(&scb.cpuid), 0xE000_ED00);
        assert_eq!(address(&scb.icsr), 0xE000_ED04);
        assert_eq!(address(&scb.vtor), 0xE000_ED08);
        assert_eq!(address(&scb.aircr), 0xE000_ED0C);
        assert_eq!(address(&scb.scr), 0xE000_ED10);
        assert_eq!(address(&scb.ccr), 0xE000_ED14);
        assert_eq!(address(&scb.shpr), 0xE000_ED18);
        assert_eq!(address(&scb.shcsr), 0xE000_ED24);
        assert_eq!(address(&scb.cfsr), 0xE000_ED28);
        assert_eq!(address(&scb.hfsr), 0xE000_ED2C);
        assert_eq!(address(&scb.dfsr), 0xE000_ED30);
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
        assert_eq!(address(&scb.clidr), 0xE000_ED78);
        assert_eq!(address(&scb.ctr), 0xE000_ED7C);
        assert_eq!(address(&scb.ccsidr), 0xE000_ED80);
        assert_eq!(address(&scb.csselr), 0xE000_ED84);
        assert_eq!(address(&scb.cpacr), 0xE000_ED88);
        assert_eq!(address(&scb.nsacr), 0xE000_ED8C);
    }

    fn address<T>(r: *const T) -> usize {
        r as usize
    }
}
