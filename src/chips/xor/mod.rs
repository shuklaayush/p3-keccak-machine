mod air;
mod columns;
mod interaction;
mod trace;

#[cfg(feature = "trace-writer")]
use p3_air_util::TraceWriter;
#[cfg(feature = "trace-writer")]
use p3_field::{ExtensionField, Field};

// TODO: Just proof of concept, should be implemented as lookup.
//       Can be extended to a general CPU chip.
#[derive(Clone, Debug)]
pub struct XorChip {
    pub bus_input: usize,
    pub bus_output: usize,
}

#[cfg(feature = "trace-writer")]
impl<F: Field, EF: ExtensionField<F>> TraceWriter<F, EF> for XorChip {
    fn main_headers(&self) -> Vec<String> {
        self::columns::XorCols::<F>::headers()
    }
}
