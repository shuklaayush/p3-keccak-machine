use itertools::Itertools;
use p3_field::PrimeField32;
use p3_matrix::dense::RowMajorMatrix;
use tracing::instrument;

use super::columns::KeccakPermuteCols;
use super::KeccakPermuteChip;
use crate::airs::keccak::{generate_trace_rows_for_perm, NUM_ROUNDS};

#[derive(Default)]
pub struct KeccakPermuteOp {
    pub input: [u64; 25],
}

impl KeccakPermuteChip {
    #[instrument(name = "generate KeccakPermute trace", skip_all)]
    pub fn generate_trace<F: PrimeField32>(ops: Vec<KeccakPermuteOp>) -> RowMajorMatrix<F> {
        let num_cols = KeccakPermuteCols::<F>::num_cols();
        let num_real_rows = ops.len() * NUM_ROUNDS;
        let num_rows = num_real_rows.next_power_of_two();
        let mut trace = RowMajorMatrix::new(vec![F::zero(); num_rows * num_cols], num_cols);
        let (prefix, rows, suffix) = unsafe { trace.values.align_to_mut::<KeccakPermuteCols<F>>() };
        assert!(prefix.is_empty(), "Alignment should match");
        assert!(suffix.is_empty(), "Alignment should match");
        assert_eq!(rows.len(), num_rows);

        let mut real_rows = rows.iter_mut().collect_vec();
        Self::populate_rows_for_ops(&mut real_rows, &ops);

        for pad_rows in rows.chunks_mut(NUM_ROUNDS).skip(ops.len()) {
            let op = KeccakPermuteOp::default();
            let mut rows_ref = pad_rows
                .iter_mut()
                .map(|row| &mut row.keccak)
                .collect::<Vec<_>>();
            generate_trace_rows_for_perm(&mut rows_ref, op.input);
        }

        trace
    }

    pub fn populate_rows_for_ops<F: PrimeField32>(
        rows: &mut [&mut KeccakPermuteCols<F>],
        ops: &[KeccakPermuteOp],
    ) {
        for (op, rows) in ops.iter().zip(rows.chunks_mut(NUM_ROUNDS)) {
            Self::populate_rows_for_op(rows, op);
        }
    }

    pub fn populate_rows_for_op<F: PrimeField32>(
        rows: &mut [&mut KeccakPermuteCols<F>],
        op: &KeccakPermuteOp,
    ) {
        debug_assert!(rows.len() == NUM_ROUNDS, "Exptected {NUM_ROUNDS} rows");
        for (i, row) in rows.iter_mut().enumerate() {
            if i < NUM_ROUNDS {
                row.is_real = F::one();
                if i % NUM_ROUNDS == 0 {
                    row.is_real_input = F::one();
                }
                if i % NUM_ROUNDS == NUM_ROUNDS - 1 {
                    row.is_real_output = F::one();
                }
            }
        }
        let mut keccak_rows = rows
            .iter_mut()
            .map(|row| &mut row.keccak)
            .collect::<Vec<_>>();
        generate_trace_rows_for_perm(&mut keccak_rows, op.input);
    }
}
