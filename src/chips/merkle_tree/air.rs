use core::borrow::Borrow;
use p3_air::{Air, AirBuilder, BaseAir};
use p3_air_util::builders::SubRangeAirBuilder;
use p3_field::AbstractField;
use p3_matrix::Matrix;

use crate::airs::step_flags::StepFlagsAir;

use super::{columns::MerkleRootCols, MerkleRootChip};

impl<F, const DEPTH: usize, const DIGEST_WIDTH: usize> BaseAir<F>
    for MerkleRootChip<DEPTH, DIGEST_WIDTH>
{
    fn width(&self) -> usize {
        MerkleRootCols::<F, DEPTH, DIGEST_WIDTH>::num_cols()
    }
}

impl<AB, const DEPTH: usize, const DIGEST_WIDTH: usize> Air<AB>
    for MerkleRootChip<DEPTH, DIGEST_WIDTH>
where
    AB: AirBuilder,
{
    fn eval(&self, builder: &mut AB) {
        let col_map = MerkleRootCols::<AB::Var, DEPTH, DIGEST_WIDTH>::col_map();

        let main = builder.main();
        let (local, next) = (main.row_slice(0), main.row_slice(1));
        let local: &MerkleRootCols<AB::Var, DEPTH, DIGEST_WIDTH> = (*local).borrow();
        let next: &MerkleRootCols<AB::Var, DEPTH, DIGEST_WIDTH> = (*next).borrow();

        let step_flags_air = StepFlagsAir::<DEPTH>;
        let mut sub_builder = SubRangeAirBuilder::new_main(builder, col_map.step_flags.as_range());
        step_flags_air.eval(&mut sub_builder);

        builder.assert_bool(local.is_real);
        builder.assert_bool(local.is_right_child);

        let is_first_step = local.step_flags.flags[0];
        let is_final_step = local.step_flags.flags[DEPTH - 1];

        // Accumulated index is computed correctly
        builder
            .when(is_first_step)
            .assert_eq(local.accumulated_index, local.is_right_child);
        let bit_factor: AB::Expr = next
            .step_flags
            .flags
            .iter()
            .enumerate()
            .map(|(i, &flag)| flag * AB::Expr::from_canonical_usize(1 << i))
            .sum();
        builder.when_ne(is_final_step, AB::Expr::one()).assert_eq(
            next.accumulated_index,
            bit_factor * next.is_right_child + local.accumulated_index,
        );

        // Left and right nodes are selected correctly.
        for i in 0..DIGEST_WIDTH {
            let diff = local.node[i] - local.sibling[i];
            let left = local.node[i] - local.is_right_child * diff.clone();
            let right = local.sibling[i] + local.is_right_child * diff;

            builder.assert_eq(left, local.left_node[i]);
            builder.assert_eq(right, local.right_node[i]);
        }

        // Output is copied to the next row.
        for i in 0..DIGEST_WIDTH {
            builder
                .when_ne(is_final_step, AB::Expr::one())
                .assert_eq(local.output[i], next.node[i]);
        }
    }
}
