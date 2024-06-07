use alloc::collections::BTreeMap;

use itertools::Itertools;
use p3_field::PrimeField32;
use p3_matrix::dense::RowMajorMatrix;

use super::{columns::RangeCols, RangeCheckerChip};

impl<const MAX: u32> RangeCheckerChip<MAX> {
    pub fn generate_trace<F: PrimeField32>(count: BTreeMap<u32, u32>) -> RowMajorMatrix<F> {
        let num_cols = RangeCols::<F>::num_cols();
        let num_real_rows = MAX as usize;
        let num_rows = num_real_rows.next_power_of_two();
        let mut trace = RowMajorMatrix::new(vec![F::zero(); num_rows * num_cols], num_cols);
        let (prefix, rows, suffix) = unsafe { trace.values.align_to_mut::<RangeCols<F>>() };
        assert!(prefix.is_empty(), "Alignment should match");
        assert!(suffix.is_empty(), "Alignment should match");
        assert_eq!(rows.len(), num_rows);

        let mut real_rows = rows[0..num_real_rows].iter_mut().collect_vec();
        Self::populate_rows_for_counts(&mut real_rows, count);

        trace
    }

    pub fn populate_rows_for_counts<F>(rows: &mut [&mut RangeCols<F>], count: BTreeMap<u32, u32>)
    where
        F: PrimeField32,
    {
        for (n, row) in rows.iter_mut().enumerate() {
            // FIXME: This is very inefficient when the range is large.
            // Iterate over key/val pairs instead in a separate loop.
            if let Some(c) = count.get(&(n as u32)) {
                row.mult = F::from_canonical_u32(*c);
            }
        }
    }
}
