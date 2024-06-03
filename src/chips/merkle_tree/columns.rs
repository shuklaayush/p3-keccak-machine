use p3_derive::Columnar;

use crate::airs::keccak::U64_LIMBS;
use crate::chips::keccak_permute::NUM_U64_HASH_ELEMS;

#[repr(C)]
#[derive(Columnar)]
pub struct MerkleRootCols<T> {
    pub is_real: T,

    pub is_final_step: T,

    pub node: [[T; U64_LIMBS]; NUM_U64_HASH_ELEMS],

    pub sibling: [[T; U64_LIMBS]; NUM_U64_HASH_ELEMS],

    pub parity_selector: T,

    pub left_node: [[T; U64_LIMBS]; NUM_U64_HASH_ELEMS],

    pub right_node: [[T; U64_LIMBS]; NUM_U64_HASH_ELEMS],

    pub output: [[T; U64_LIMBS]; NUM_U64_HASH_ELEMS],
}
