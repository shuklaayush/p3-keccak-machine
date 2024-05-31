use p3_derive::{AirColumns, AlignedBorrow};

#[repr(C)]
#[derive(AlignedBorrow, AirColumns)]
pub struct XorCols<T> {
    pub is_real: T,

    pub input1: [T; 4],

    pub input2: [T; 4],

    /// Bit decomposition of input_1 bytes
    pub bits1: [[T; 8]; 4],

    /// Bit decomposition of input_2 bytes
    pub bits2: [[T; 8]; 4],

    /// Aggregated output
    pub output: [T; 4],
}
