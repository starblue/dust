/// PLU peripheral unit for LPC804
///
/// Checked against UM11065 2018-03-21.
use volatile_register::{RO, RW};

pub const PLU_INPUTS: usize = 6;
pub const PLU_OUTPUTS: usize = 8;
pub const LUTS: usize = 26;
pub const LUT_INPUTS: usize = 5;

#[repr(C)]
pub struct Plu {
    pub lut_inp_mux: [[RW<u32>; 8]; 64],
    pub lut_truth: [RW<u32>; 64],
    pub outputs: RO<u32>,
    reserved_0x0904: [u8; 0xc00 - 0x904],
    pub output_mux: [RW<u32>; PLU_OUTPUTS],
}

pub enum LutInputMux {
    PluInput(usize),
    LutOutput(usize),
    State(usize),
}

pub enum OutputMux {
    LutOutput(usize),
    State(usize),
}

impl Plu {
    pub unsafe fn set_lut_input(&mut self, lut: usize, input: usize, mux: LutInputMux) {
        self.lut_inp_mux[lut][input].write(match mux {
            LutInputMux::PluInput(n) => n,
            LutInputMux::LutOutput(n) => n + PLU_INPUTS,
            LutInputMux::State(n) => n + PLU_INPUTS + LUTS,
        } as u32);
    }

    pub unsafe fn set_output(&mut self, output: usize, mux: OutputMux) {
        self.output_mux[output].write(match mux {
            OutputMux::LutOutput(n) => n,
            OutputMux::State(n) => n + LUTS,
        } as u32);
    }
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
