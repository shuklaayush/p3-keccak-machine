use p3_derive::AirColumns;

use crate::airs::keccak::KeccakCols;

#[repr(C)]
#[derive(AirColumns)]
pub struct KeccakPermuteCols<T> {
    pub keccak: KeccakCols<T>,

    pub is_real: T,

    pub is_real_input: T,

    pub is_real_output_full: T,

    pub is_real_output_digest: T,
}
