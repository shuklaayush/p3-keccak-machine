mod air;
mod columns;
mod interaction;
mod trace;

use alloc::vec::Vec;
use p3_air_util::TraceWriter;
use p3_field::{ExtensionField, PrimeField32};

use self::columns::XorCols;

// TODO: Just proof of concept, should be implemented as lookup.
//       Can be extended to a general CPU chip.
#[derive(Clone, Debug)]
pub struct XorChip {
    pub bus_xor_input: usize,
    pub bus_xor_output: usize,
}

#[cfg(feature = "trace-writer")]
impl<F: PrimeField32, EF: ExtensionField<F>> TraceWriter<F, EF> for XorChip {
    fn main_headers(&self) -> Vec<String> {
        XorCols::<F>::headers()
    }
}
