use p3_derive::Columnar;

use crate::airs::keccak::KeccakCols;

#[repr(C)]
#[derive(Columnar)]
pub struct KeccakPermuteCols<T> {
    pub keccak: KeccakCols<T>,

    pub is_real: T,

    pub is_real_input: T,

    pub is_real_output_full: T,

    pub is_real_output_digest: T,
}
