/// PLU peripheral unit for LPC804
///
/// Checked against UM11065 2018-03-21.
use volatile_register::{RO, RW};

pub const LUTS: usize = 26;
pub const LUT_INPUTS: usize = 5;

#[repr(C)]
pub struct Plu {
    pub lut_inp_mux: [[RW<u32>; 8]; 64],
    pub lut_truth: [RW<u32>; 64],
    pub outputs:  RO<u32>,
    reserved_0x0904: [u8; 0xc00 - 0x904],
    pub output_mux: [RW<u32>; 8],
}

#[cfg(test)]
mod test {
    #[test]
    fn test_plu() {
        let plu = unsafe { &mut *::PLU };

        assert_eq!(address(&plu.lut_inp_mux), 0x4002_8000);
        assert_eq!(address(&plu.lut_truth), 0x4002_8800);
        assert_eq!(address(&plu.outputs), 0x4002_8900);
        assert_eq!(address(&plu.output_mux), 0x4002_8c00);
    }

    fn address<T>(r: *const T) -> usize {
        r as usize
    }
}
