use p3_derive::{AirColumns, AlignedBorrow};
use p3_keccak_air::U64_LIMBS;

use crate::chips::keccak_permute::NUM_U64_HASH_ELEMS;

#[repr(C)]
#[derive(AlignedBorrow, AirColumns)]
pub struct MerkleTreeCols<T> {
    pub is_real: T,

    pub is_final_step: T,

    pub node: [[T; U64_LIMBS]; NUM_U64_HASH_ELEMS],

    pub sibling: [[T; U64_LIMBS]; NUM_U64_HASH_ELEMS],

    pub parity_selector: T,

    pub left_node: [[T; U64_LIMBS]; NUM_U64_HASH_ELEMS],

    pub right_node: [[T; U64_LIMBS]; NUM_U64_HASH_ELEMS],

    pub output: [[T; U64_LIMBS]; NUM_U64_HASH_ELEMS],
}
