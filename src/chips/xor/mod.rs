mod air;
mod columns;
mod interaction;
pub mod trace;

// TODO: Just proof of concept, should be implemented as lookup.
//       Can be extended to a general CPU chip.
#[derive(Clone, Debug)]
pub struct XorChip<const NUM_BYTES: usize> {
    pub bus_input: usize,
    pub bus_output: usize,
}

#[cfg(feature = "air-logger")]
impl<const NUM_BYTES: usize> p3_air_util::AirLogger for XorChip<NUM_BYTES> {
    fn main_headers(&self) -> Vec<String> {
        self::columns::XorCols::<usize, NUM_BYTES>::headers()
    }

    #[cfg(feature = "schema")]
    fn main_headers_and_types(&self) -> Vec<(String, String, core::ops::Range<usize>)> {
        self::columns::XorCols::<usize, NUM_BYTES>::headers_and_types()
    }
}
