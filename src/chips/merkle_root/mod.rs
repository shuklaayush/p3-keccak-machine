mod air;
mod columns;
mod interaction;
mod trace;

pub use trace::MerkleRootOp;

#[derive(Default, Clone, Debug)]
pub struct MerkleRootChip<const DEPTH: usize, const DIGEST_WIDTH: usize> {
    pub bus_hasher_input: usize,
    pub bus_hasher_output: usize,
}

#[cfg(feature = "air-logger")]
impl<const DEPTH: usize, const DIGEST_WIDTH: usize> p3_air_util::AirLogger
    for MerkleRootChip<DEPTH, DIGEST_WIDTH>
{
    fn main_headers(&self) -> Vec<String> {
        self::columns::MerkleRootCols::<usize, DEPTH, DIGEST_WIDTH>::headers()
    }
    #[cfg(feature = "schema")]
    fn main_headers_and_types(&self) -> Vec<(String, String, core::ops::Range<usize>)> {
        self::columns::MerkleRootCols::<usize, DEPTH, DIGEST_WIDTH>::headers_and_types()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::prove_and_verify;

    use itertools::Itertools;
    use p3_keccak::Keccak256Hash;
    use p3_symmetric::{CompressionFunction, CompressionFunctionFromHasher};
    use p3_uni_stark::VerificationError;
    use rand::{rngs::StdRng, Rng, SeedableRng};

    fn generate_digests<Compress: CompressionFunction<[u8; 32], 2>>(
        leaf_hashes: Vec<[u8; 32]>,
        hasher: &Compress,
    ) -> Vec<Vec<[u8; 32]>> {
        let mut digests = vec![leaf_hashes];

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
    fn test_merkle_root_prove() -> Result<(), VerificationError> {
        const RANDOM_SEED: u64 = 0;
        let mut seeded_rng = StdRng::seed_from_u64(RANDOM_SEED);

        const HEIGHT: usize = 3;
        const NUM_LEAVES: usize = 1 << HEIGHT;

        let hasher = CompressionFunctionFromHasher::new(Keccak256Hash);

        let leaf_hashes = (0..NUM_LEAVES).map(|_| seeded_rng.gen()).collect_vec();
        let digests = generate_digests(leaf_hashes, &hasher);

        let leaf_index = seeded_rng.gen_range(0..NUM_LEAVES);
        let leaf_hash = digests[0][leaf_index];

        let siblings: [[u8; 32]; HEIGHT] = (0..HEIGHT)
            .map(|i| digests[i][(leaf_index >> i) ^ 1])
            .collect::<Vec<[u8; 32]>>()
            .try_into()
            .unwrap();
        let op = MerkleRootOp {
            leaf_index,
            leaf_hash,
            siblings,
        };

        let trace = MerkleRootChip::generate_trace(vec![op], &hasher);

        let chip: MerkleRootChip<HEIGHT, 32> = MerkleRootChip {
            ..Default::default()
        };

        prove_and_verify(&chip, trace, vec![])
    }
}
