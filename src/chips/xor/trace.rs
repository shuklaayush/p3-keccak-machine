use itertools::Itertools;
use p3_field::PrimeField32;
use p3_matrix::dense::RowMajorMatrix;

use super::{columns::XorCols, XorChip};

pub struct XorOp {
    pub input1: u16,
    pub input2: u16,
}

impl<const NUM_BYTES: usize> XorChip<NUM_BYTES> {
    pub fn generate_trace<F: PrimeField32>(operations: Vec<XorOp>) -> RowMajorMatrix<F> {
        let num_cols = XorCols::<F, NUM_BYTES>::num_cols();
        let num_real_rows = operations.len();
        let num_rows = num_real_rows.next_power_of_two();
        let mut trace = RowMajorMatrix::new(vec![F::zero(); num_rows * num_cols], num_cols);

        let (prefix, rows, suffix) =
            unsafe { trace.values.align_to_mut::<XorCols<F, NUM_BYTES>>() };
        assert!(prefix.is_empty(), "Alignment should match");
        assert!(suffix.is_empty(), "Alignment should match");
        assert_eq!(rows.len(), num_rows);

        let mut real_rows = rows[0..num_real_rows].iter_mut().collect_vec();
        Self::populate_rows_for_ops(&mut real_rows, &operations);

        trace
    }

    pub fn populate_rows_for_ops<F: PrimeField32>(
        rows: &mut [&mut XorCols<F, NUM_BYTES>],
        ops: &[XorOp],
    ) {
        for (row, op) in rows.iter_mut().zip(ops.iter()) {
            Self::populate_row_for_op(row, op);
        }
    }

    pub fn populate_row_for_op<F: PrimeField32>(row: &mut XorCols<F, NUM_BYTES>, op: &XorOp) {
        row.is_real = F::one();

        let input1_bytes = op.input1.to_le_bytes();
        let input2_bytes = op.input2.to_le_bytes();
        for (i, (input1, input2)) in input1_bytes
            .into_iter()
            .zip(input2_bytes.into_iter())
            .enumerate()
        {
            row.input1[i] = F::from_canonical_u8(input1);
            row.input2[i] = F::from_canonical_u8(input2);
            row.output[i] = F::from_canonical_u8(input1 ^ input2);

            for j in 0..8 {
                row.bits1[i][j] = F::from_canonical_u8(input1 >> j & 1);
                row.bits2[i][j] = F::from_canonical_u8(input2 >> j & 1);
            }
        }
    }
}
