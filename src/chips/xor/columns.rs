use p3_derive::Columnar;

#[repr(C)]
#[derive(Columnar)]
pub struct XorCols<T, const NUM_BYTES: usize> {
    pub is_real: T,

    pub input1: [T; NUM_BYTES],

    pub input2: [T; NUM_BYTES],

    /// Bit decomposition of input_1 bytes
    pub bits1: [[T; 8]; NUM_BYTES],

    /// Bit decomposition of input_2 bytes
    pub bits2: [[T; 8]; NUM_BYTES],

    /// Aggregated output
    pub output: [T; NUM_BYTES],
}
