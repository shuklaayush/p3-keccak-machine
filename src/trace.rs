use itertools::Itertools;
use p3_field::PrimeField32;
use p3_matrix::dense::RowMajorMatrix;
use p3_symmetric::CompressionFunction;
use p3_uni_stark::{StarkGenericConfig, Val};

use crate::chips::{
    keccak_permute::{trace::KeccakPermuteOp, KeccakPermuteChip},
    keccak_sponge::{
        columns::{KECCAK_RATE_BYTES, KECCAK_WIDTH_BYTES},
        trace::KeccakSpongeOp,
        KeccakSpongeChip,
    },
    merkle_root::{MerkleRootChip, MerkleRootOp},
    xor::XorChip,
    DIGEST_WIDTH, MERKLE_TREE_DEPTH,
};

// TODO: Proper execution function for the machine that minimizes redundant computation
// Store logs/events during execution first and then generate the traces
pub fn generate_machine_trace<SC, Compress>(
    leaf_index: usize,
    digests: Vec<Vec<[u8; DIGEST_WIDTH]>>,
    hasher: &Compress,
) -> Vec<Option<RowMajorMatrix<Val<SC>>>>
where
    SC: StarkGenericConfig,
    Compress: CompressionFunction<[u8; DIGEST_WIDTH], 2>,
    Val<SC>: PrimeField32,
{
    let leaf_hash = digests[0][leaf_index];
    let siblings: [[u8; DIGEST_WIDTH]; MERKLE_TREE_DEPTH] = (0..MERKLE_TREE_DEPTH)
        .map(|i| {
            let depth_index = leaf_index >> i;
            digests[i][depth_index ^ 1]
        })
        .collect::<Vec<[u8; DIGEST_WIDTH]>>()
        .try_into()
        .unwrap();
    let op = MerkleRootOp {
        leaf_index,
        leaf_hash,
        siblings,
    };

    let keccak_inputs = (0..MERKLE_TREE_DEPTH)
        .map(|i| {
            let index = leaf_index >> i;
            let parity = index & 1;
            let (left, right) = if parity == 0 {
                (digests[i][index], digests[i][index ^ 1])
            } else {
                (digests[i][index ^ 1], digests[i][index])
            };
            let input = left.into_iter().chain(right).collect_vec();
            KeccakSpongeOp {
                timestamp: 0,
                addr: 0,
                input,
            }
        })
        .collect_vec();

    let merkle_tree_trace =
        MerkleRootChip::<MERKLE_TREE_DEPTH, DIGEST_WIDTH>::generate_trace(vec![op], hasher);

    let mut xor_inputs = Vec::new();
    let permute_inputs = keccak_inputs
        .iter()
        .map(|op| {
            let mut bytes_input = [0; KECCAK_WIDTH_BYTES];
            bytes_input[0..2 * DIGEST_WIDTH].copy_from_slice(&op.input);
            bytes_input[2 * DIGEST_WIDTH] = 1;
            bytes_input[KECCAK_RATE_BYTES - 1] |= 0b10000000;

            bytes_input[0..KECCAK_RATE_BYTES].chunks(4).for_each(|val| {
                xor_inputs.push((val.try_into().unwrap(), [0; 4]));
            });
            let input = bytes_input
                .chunks_exact(8)
                .map(|c| u64::from_le_bytes(c.try_into().unwrap()))
                .collect_vec()
                .try_into()
                .unwrap();
            KeccakPermuteOp { input }
        })
        .collect_vec();
    let keccak_sponge_trace = KeccakSpongeChip::generate_trace(keccak_inputs);

    let keccak_permute_trace = KeccakPermuteChip::generate_trace(permute_inputs);

    let xor_trace = XorChip::generate_trace(xor_inputs);

    vec![
        Some(merkle_tree_trace),
        Some(keccak_sponge_trace),
        Some(xor_trace),
        Some(keccak_permute_trace),
    ]
}
