use core::fmt::Debug;
use p3_derive::EnumDispatch;

pub mod keccak_permute;
pub mod keccak_sponge;
pub mod memory;
pub mod merkle_root;
pub mod range_checker;
pub mod xor;

use self::{
    keccak_permute::KeccakPermuteChip, keccak_sponge::KeccakSpongeChip, memory::MemoryChip,
    merkle_root::MerkleRootChip, range_checker::RangeCheckerChip, xor::XorChip,
};

pub const MERKLE_TREE_DEPTH: usize = 8;
pub const DIGEST_WIDTH: usize = 32;
pub const MAX_U8: u32 = 256;
pub const NUM_BYTES: usize = 2;

#[derive(Clone, Debug, EnumDispatch)]
pub enum KeccakMachineChip {
    KeccakPermute(KeccakPermuteChip),
    KeccakSponge(KeccakSpongeChip),
    MerkleRoot(MerkleRootChip<MERKLE_TREE_DEPTH, DIGEST_WIDTH>),
    Range8(RangeCheckerChip<MAX_U8>),
    Xor(XorChip<2>),
    Memory(MemoryChip),
}
