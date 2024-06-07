use alloc::vec::Vec;

use p3_machine::machine::Machine;

use crate::{
    bus::KeccakMachineBus,
    chips::{
        keccak_permute::KeccakPermuteChip, keccak_sponge::KeccakSpongeChip,
        merkle_root::MerkleRootChip, xor::XorChip, KeccakMachineChip,
    },
};

pub struct KeccakMachine;

impl Machine for KeccakMachine {
    type Chip = KeccakMachineChip;
    type Bus = KeccakMachineBus;

    fn chips(&self) -> Vec<KeccakMachineChip> {
        let merkle_tree_chip = MerkleRootChip {
            bus_hasher_input: KeccakMachineBus::KeccakSpongeInput as usize,
            bus_hasher_output: KeccakMachineBus::KeccakSpongeOutput as usize,
        };
        let keccak_sponge_chip = KeccakSpongeChip {
            bus_input: KeccakMachineBus::KeccakSpongeInput as usize,
            bus_output: KeccakMachineBus::KeccakSpongeOutput as usize,
            bus_permute_input: KeccakMachineBus::KeccakPermuteInput as usize,
            bus_permute_output: KeccakMachineBus::KeccakPermuteOutput as usize,
            bus_xor_input: KeccakMachineBus::XorInput as usize,
            bus_xor_output: KeccakMachineBus::XorOutput as usize,
        };
        // let range_chip = RangeCheckerChip {
        //     bus_range_8: KeccakMachineBus::Range8 as usize,
        // };
        let xor_chip = XorChip {
            bus_input: KeccakMachineBus::XorInput as usize,
            bus_output: KeccakMachineBus::XorOutput as usize,
        };
        let keccak_permute_chip = KeccakPermuteChip {
            bus_input: KeccakMachineBus::KeccakPermuteInput as usize,
            bus_output: KeccakMachineBus::KeccakPermuteOutput as usize,
        };
        // let memory_chip = MemoryChip {
        //     bus_memory: KeccakMachineBus::Memory as usize,
        //     bus_range_8: KeccakMachineBus::Range8 as usize,
        // };

        vec![
            KeccakMachineChip::MerkleRoot(merkle_tree_chip),
            KeccakMachineChip::KeccakSponge(keccak_sponge_chip),
            KeccakMachineChip::Xor(xor_chip),
            KeccakMachineChip::KeccakPermute(keccak_permute_chip),
            // KeccakMachineChip::Range8(range_chip),
            // KeccakMachineChip::Memory(memory_chip),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        chips::{DIGEST_WIDTH, MERKLE_TREE_DEPTH},
        config::{default_challenger, default_config, MyConfig},
        trace::generate_machine_trace,
    };

    use itertools::Itertools;
    use p3_keccak::Keccak256Hash;
    use p3_machine::error::VerificationError;
    use p3_symmetric::{CompressionFunction, CompressionFunctionFromHasher};
    use rand::{rngs::StdRng, Rng, SeedableRng};
    use tracing_forest::{util::LevelFilter, ForestLayer};
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry};

    fn generate_digests<Compress: CompressionFunction<[u8; DIGEST_WIDTH], 2>>(
        leaf_hashes: &[[u8; DIGEST_WIDTH]],
        hasher: &Compress,
    ) -> Vec<Vec<[u8; DIGEST_WIDTH]>> {
        let mut digests = vec![leaf_hashes.to_vec()];

        while let Some(last_level) = digests.last().cloned() {
            if last_level.len() == 1 {
                break;
            }

            let next_level = last_level
                .chunks_exact(2)
                .map(|chunk| hasher.compress([chunk[0], chunk[1]]))
                .collect();

            digests.push(next_level);
        }

        digests
    }

    #[test]
    fn test_machine_prove() -> Result<(), VerificationError> {
        const RANDOM_SEED: u64 = 0;
        let mut seeded_rng = StdRng::seed_from_u64(RANDOM_SEED);

        let env_filter = EnvFilter::builder()
            .with_default_directive(LevelFilter::INFO.into())
            .from_env_lossy();
        Registry::default()
            .with(env_filter)
            .with(ForestLayer::default())
            .init();

        const NUM_LEAVES: usize = 1 << MERKLE_TREE_DEPTH;

        let hasher = CompressionFunctionFromHasher::new(Keccak256Hash);
        let leaf_hashes = (0..NUM_LEAVES).map(|_| seeded_rng.gen()).collect_vec();
        let digests = generate_digests(&leaf_hashes, &hasher);

        let leaf_index = seeded_rng.gen_range(0..NUM_LEAVES);
        let machine = KeccakMachine;

        let (pk, vk) = machine.setup(&default_config());

        let config = default_config();
        let mut challenger = default_challenger();
        let traces = generate_machine_trace::<MyConfig, _>(leaf_index, digests, &hasher);
        let proof = machine.prove(&config, &mut challenger, &pk, traces, &[]);

        let mut challenger = default_challenger();
        machine.verify(&config, &mut challenger, &vk, &proof, &[])
    }
}
