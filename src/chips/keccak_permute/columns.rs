use p3_derive::{AirColumns, AlignedBorrow};
use p3_keccak_air::KeccakCols;

#[repr(C)]
#[derive(AlignedBorrow, AirColumns)]
pub struct KeccakPermuteCols<T> {
    pub keccak: KeccakCols<T>,

    pub is_real: T,

    pub is_real_input: T,

    pub is_real_output: T,

    pub is_real_digest: T,
}
