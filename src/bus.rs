use p3_field::PrimeField32;
use p3_machine::machine::Machine;
use p3_uni_stark::{StarkGenericConfig, Val};

use crate::{
    chips::KeccakMachineChip,
    chips::{
        keccak_permute::KeccakPermuteChip, keccak_sponge::KeccakSpongeChip, memory::MemoryChip,
        merkle_tree::MerkleRootChip, range_checker::RangeCheckerChip, xor::XorChip,
    },
};

pub struct KeccakMachine {}

pub enum KeccakMachineBus {
    KeccakPermuteInput = 0,
    KeccakPermuteOutputFull = 1,
    KeccakPermuteOutputDigest = 2,
    Range8 = 3,
    XorInput = 4,
    XorOutput = 5,
    Memory = 6,
}
