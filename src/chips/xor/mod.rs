mod air;
mod columns;
mod interaction;
mod trace;

// TODO: Just proof of concept, should be implemented as lookup.
//       Can be extended to a general CPU chip.
#[derive(Clone, Debug)]
pub struct XorChip {
    pub bus_input: usize,
    pub bus_output: usize,
}

#[cfg(feature = "air-logger")]
impl p3_air_util::AirLogger for XorChip {
    fn main_headers(&self) -> Vec<String> {
        self::columns::XorCols::<usize>::headers()
    }

    #[cfg(feature = "schema")]
    fn main_headers_and_types(&self) -> Vec<(String, String, core::ops::Range<usize>)> {
        self::columns::XorCols::<usize>::headers_and_types()
    }
}
