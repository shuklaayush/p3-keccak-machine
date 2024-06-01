use p3_field::PrimeField32;
use p3_matrix::dense::RowMajorMatrix;
use p3_matrix::Matrix;
use tracing::instrument;

use super::columns::KeccakPermuteCols;
use super::KeccakPermuteChip;
use crate::airs::keccak::{generate_trace_rows, NUM_KECCAK_COLS, NUM_ROUNDS};

pub enum KeccakPermuteOpType {
    Full,
    Digest,
}

pub struct KeccakPermuteOp {
    pub op_type: KeccakPermuteOpType,
    pub input: [u64; 25],
}

impl KeccakPermuteChip {
    #[instrument(name = "generate KeccakPermute trace", skip_all)]
    pub fn generate_trace<F: PrimeField32>(ops: Vec<KeccakPermuteOp>) -> RowMajorMatrix<F> {
        let num_cols = KeccakPermuteCols::<F>::num_cols();
        let col_map = KeccakPermuteCols::<F>::col_map();

        let num_inputs = ops.len();
        let inputs = ops.iter().map(|op| op.input).collect::<Vec<_>>();
        let keccak_trace: RowMajorMatrix<F> = generate_trace_rows(inputs);

        let mut trace =
            RowMajorMatrix::new(vec![F::zero(); keccak_trace.height() * num_cols], num_cols);
        for i in 0..keccak_trace.height() {
            // TODO: Better way to do this, ideally the inner trace would be generated on &mut rows
            trace.row_mut(i)[..NUM_KECCAK_COLS].copy_from_slice(&keccak_trace.row_slice(i));
        }

        for (i, row) in trace.rows_mut().enumerate() {
            if i < num_inputs * NUM_ROUNDS {
                row[col_map.is_real] = F::one();
                if i % NUM_ROUNDS == 0 {
                    row[col_map.is_real_input] = F::one();
                }
                if i % NUM_ROUNDS == NUM_ROUNDS - 1 {
                    row[col_map.is_real_output] = F::one();
                }
            }
        }

        trace
    }
}
