use itertools::Itertools;
use p3_field::PrimeField32;
use p3_matrix::{dense::RowMajorMatrix, Matrix};
use p3_symmetric::CompressionFunction;
use tracing::instrument;

use super::{columns::MerkleRootCols, MerkleRootChip};

#[derive(Clone)]
pub struct MerkleRootOp<T, const DEPTH: usize, const DIGEST_WIDTH: usize>
where
    T: Default + Copy,
{
    pub leaf_index: usize,
    pub leaf_hash: [T; DIGEST_WIDTH],
    pub siblings: [[T; DIGEST_WIDTH]; DEPTH],
}

impl<T, const DEPTH: usize, const DIGEST_WIDTH: usize> Default
    for MerkleRootOp<T, DEPTH, DIGEST_WIDTH>
where
    T: Default + Copy,
{
    fn default() -> Self {
        Self {
            leaf_index: 0,
            leaf_hash: [T::default(); DIGEST_WIDTH],
            siblings: [[T::default(); DIGEST_WIDTH]; DEPTH],
        }
    }
}

impl<const DEPTH: usize, const DIGEST_WIDTH: usize> MerkleRootChip<DEPTH, DIGEST_WIDTH> {
    // TODO: Allow empty traces
    #[instrument(name = "generate MerkleRootChip trace", skip_all)]
    pub fn generate_trace<F, T, Compress>(
        operations: Vec<MerkleRootOp<T, DEPTH, DIGEST_WIDTH>>,
        hasher: &Compress,
    ) -> RowMajorMatrix<F>
    where
        F: PrimeField32,
        T: Default + Copy + Into<u32>,
        Compress: CompressionFunction<[T; DIGEST_WIDTH], 2>,
    {
        let num_cols = MerkleRootCols::<F, DEPTH, DIGEST_WIDTH>::num_cols();

        let num_real_rows = operations.len() * DEPTH;
        let num_rows = num_real_rows.next_power_of_two();
        let mut trace = RowMajorMatrix::new(vec![F::zero(); num_rows * num_cols], num_cols);
        let (prefix, rows, suffix) = unsafe {
            trace
                .values
                .align_to_mut::<MerkleRootCols<F, DEPTH, DIGEST_WIDTH>>()
        };
        assert!(prefix.is_empty(), "Alignment should match");
        assert!(suffix.is_empty(), "Alignment should match");
        assert_eq!(rows.len(), num_rows);

        for (leaf_rows, op) in rows.chunks_mut(DEPTH).zip(operations.iter()) {
            generate_trace_rows_for_op(leaf_rows, op, hasher);

            for row in leaf_rows.iter_mut() {
                row.is_real = F::one();
            }
        }

        // Fill padding rows
        for input_rows in rows.chunks_mut(DEPTH).skip(operations.len()) {
            let op = MerkleRootOp::default();
            generate_trace_rows_for_op(input_rows, &op, hasher);
        }

        trace
    }
}

pub fn generate_trace_rows_for_op<F, T, Compress, const DEPTH: usize, const DIGEST_WIDTH: usize>(
    rows: &mut [MerkleRootCols<F, DEPTH, DIGEST_WIDTH>],
    op: &MerkleRootOp<T, DEPTH, DIGEST_WIDTH>,
    hasher: &Compress,
) where
    F: PrimeField32,
    T: Default + Copy + Into<u32>,
    Compress: CompressionFunction<[T; DIGEST_WIDTH], 2>,
{
    let MerkleRootOp {
        leaf_index,
        leaf_hash,
        siblings,
    } = op;

    // Fill the first row with the leaf.
    for (node_byte, &leaf_hash_byte) in rows[0].node.iter_mut().zip(leaf_hash.iter()) {
        *node_byte = F::from_canonical_u32(leaf_hash_byte.into());
    }

    let mut node = generate_trace_row_for_round(
        &mut rows[0],
        0,
        leaf_index & 1,
        leaf_index & 1,
        leaf_hash,
        &siblings[0],
        hasher,
    );

    for round in 1..rows.len() {
        // Copy previous row's output to next row's input.
        for i in 0..DIGEST_WIDTH {
            rows[round].node[i] = rows[round - 1].output[i];
        }

        let mask = (1 << (round + 1)) - 1;
        node = generate_trace_row_for_round(
            &mut rows[round],
            round,
            leaf_index & mask,
            (leaf_index >> round) & 1,
            &node,
            &siblings[round],
            hasher,
        );
    }
}

pub fn generate_trace_row_for_round<F, T, Compress, const DEPTH: usize, const DIGEST_WIDTH: usize>(
    row: &mut MerkleRootCols<F, DEPTH, DIGEST_WIDTH>,
    round: usize,
    accumulated_index: usize,
    is_right_child: usize,
    node: &[T; DIGEST_WIDTH],
    sibling: &[T; DIGEST_WIDTH],
    hasher: &Compress,
) -> [T; DIGEST_WIDTH]
where
    F: PrimeField32,
    T: Default + Copy + Into<u32>,
    Compress: CompressionFunction<[T; DIGEST_WIDTH], 2>,
{
    row.step_flags.flags[round] = F::one();

    let (left_node, right_node) = if is_right_child == 0 {
        (node, sibling)
    } else {
        (sibling, node)
    };

    let output = hasher.compress([*left_node, *right_node]);

    row.is_right_child = F::from_canonical_usize(is_right_child);
    row.accumulated_index = F::from_canonical_usize(accumulated_index);
    for i in 0..DIGEST_WIDTH {
        row.sibling[i] = F::from_canonical_u32(sibling[i].into());

        row.left_node[i] = F::from_canonical_u32(left_node[i].into());
        row.right_node[i] = F::from_canonical_u32(right_node[i].into());

        row.output[i] = F::from_canonical_u32(output[i].into());
    }

    output
}
