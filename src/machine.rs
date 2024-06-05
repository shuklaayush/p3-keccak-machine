use alloc::vec::Vec;

use p3_machine::machine::Machine;

use crate::{
    bus::KeccakMachineBus,
    chips::{
        keccak_permute::KeccakPermuteChip, keccak_sponge::KeccakSpongeChip, memory::MemoryChip,
        merkle_tree::MerkleRootChip, range_checker::RangeCheckerChip, xor::XorChip,
        KeccakMachineChip,
    },
};

pub struct KeccakMachine;

impl Machine for KeccakMachine {
    type Chip = KeccakMachineChip;
    type Bus = KeccakMachineBus;

    fn chips(&self) -> Vec<KeccakMachineChip> {
        let keccak_permute_chip = KeccakPermuteChip {
            bus_input: KeccakMachineBus::KeccakPermuteInputBus as usize,
            bus_output_full: KeccakMachineBus::KeccakPermuteOutputFullBus as usize,
            bus_output_digest: KeccakMachineBus::KeccakPermuteOutputDigestBus as usize,
        };
        let keccak_sponge_chip = KeccakSpongeChip {
            bus_xor_input: KeccakMachineBus::XorInputBus as usize,
            bus_xor_output: KeccakMachineBus::XorOutputBus as usize,
            bus_permute_input: KeccakMachineBus::KeccakPermuteInputBus as usize,
            bus_permute_output: KeccakMachineBus::KeccakPermuteOutputFullBus as usize,
            bus_range_8: KeccakMachineBus::Range8Bus as usize,
            bus_memory: KeccakMachineBus::MemoryBus as usize,
        };
        let merkle_tree_chip = MerkleRootChip {
            bus_input: KeccakMachineBus::KeccakPermuteInputBus as usize,
            bus_output: KeccakMachineBus::KeccakPermuteOutputDigestBus as usize,
        };
        let range_chip = RangeCheckerChip {
            bus_range_8: KeccakMachineBus::Range8Bus as usize,
        };
        let xor_chip = XorChip {
            bus_input: KeccakMachineBus::XorInputBus as usize,
            bus_output: KeccakMachineBus::XorOutputBus as usize,
        };
        let memory_chip = MemoryChip {
            bus_memory: KeccakMachineBus::MemoryBus as usize,
            bus_range_8: KeccakMachineBus::Range8Bus as usize,
        };

        vec![
            KeccakMachineChip::KeccakPermute(keccak_permute_chip),
            KeccakMachineChip::KeccakSponge(keccak_sponge_chip),
            KeccakMachineChip::MerkleTree(merkle_tree_chip),
            KeccakMachineChip::Range8(range_chip),
            KeccakMachineChip::Xor(xor_chip),
            KeccakMachineChip::Memory(memory_chip),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        config::{default_challenger, default_config, MyConfig},
        trace::generate_machine_trace,
    };

    use itertools::Itertools;
    use p3_keccak::KeccakF;
    use p3_machine::error::VerificationError;
    use p3_symmetric::{PseudoCompressionFunction, TruncatedPermutation};
    use rand::{random, thread_rng, Rng};
    use tracing_forest::{util::LevelFilter, ForestLayer};
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry};

    fn generate_digests(leaf_hashes: &[[u8; 32]]) -> Vec<Vec<[u8; 32]>> {
        let keccak = TruncatedPermutation::new(KeccakF {});
        let mut digests = vec![leaf_hashes.to_vec()];

        while let Some(last_level) = digests.last().cloned() {
            if last_level.len() == 1 {
                break;
            }

            let next_level = last_level
                .chunks_exact(2)
                .map(|chunk| keccak.compress([chunk[0], chunk[1]]))
                .collect();

            digests.push(next_level);
        }

        digests
    }

    #[test]
    fn test_machine_prove() -> Result<(), VerificationError> {
        let env_filter = EnvFilter::builder()
            .with_default_directive(LevelFilter::INFO.into())
            .from_env_lossy();

        Registry::default()
            .with(env_filter)
            .with(ForestLayer::default())
            .init();

        const NUM_BYTES: usize = 1000;
        let preimage = (0..NUM_BYTES).map(|_| random()).collect_vec();

        const HEIGHT: usize = 8;
        let leaf_hashes = (0..2u64.pow(HEIGHT as u32)).map(|_| random()).collect_vec();
        let digests = generate_digests(&leaf_hashes);

        let leaf_index = thread_rng().gen_range(0..leaf_hashes.len());
        let machine = KeccakMachine {};

        let (pk, vk) = machine.setup(&default_config());

        let config = default_config();
        let mut challenger = default_challenger();
        let traces = generate_machine_trace::<MyConfig>(preimage, digests, leaf_index);
        let proof = machine.prove(&config, &mut challenger, &pk, traces, &[]);

        let mut challenger = default_challenger();
        machine.verify(&config, &mut challenger, &vk, &proof, &[])
    }
}
