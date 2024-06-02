use core::borrow::Borrow;
use p3_air::{Air, AirBuilder, BaseAir};
use p3_field::AbstractField;
use p3_matrix::Matrix;

use super::{columns::MerkleRootCols, MerkleRootChip};
use crate::airs::keccak::U64_LIMBS;
use crate::chips::keccak_permute::NUM_U64_HASH_ELEMS;

impl<F> BaseAir<F> for MerkleRootChip {
    fn width(&self) -> usize {
        MerkleRootCols::<F>::num_cols()
    }
}

impl<AB: AirBuilder> Air<AB> for MerkleRootChip {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local = main.row_slice(0);
        let next = main.row_slice(1);
        let local: &MerkleRootCols<AB::Var> = (*local).borrow();
        let next: &MerkleRootCols<AB::Var> = (*next).borrow();

        // TODO: Add more constraints.
        builder.assert_bool(local.is_real);

        // Left and right nodes are selected correctly.
        for i in 0..NUM_U64_HASH_ELEMS {
            for j in 0..U64_LIMBS {
                let diff = local.node[i][j] - local.sibling[i][j];
                let left = local.node[i][j] - local.parity_selector * diff.clone();
                let right = local.sibling[i][j] + local.parity_selector * diff;

                builder.assert_eq(left, local.left_node[i][j]);
                builder.assert_eq(right, local.right_node[i][j]);
            }
        }

        // Output is copied to the next row.
        for i in 0..NUM_U64_HASH_ELEMS {
            for j in 0..U64_LIMBS {
                builder
                    .when(AB::Expr::one() - local.is_final_step)
                    .assert_eq(local.output[i][j], next.node[i][j]);
            }
        }
    }
}
