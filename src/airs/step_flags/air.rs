use core::borrow::Borrow;
use p3_air::{Air, AirBuilder, BaseAir};
use p3_matrix::Matrix;

use super::columns::StepFlagsCols;

pub struct StepFlagsAir<const N: usize>;

impl<F, const N: usize> BaseAir<F> for StepFlagsAir<N> {
    fn width(&self) -> usize {
        StepFlagsCols::<F, N>::num_cols()
    }
}

impl<AB: AirBuilder, const N: usize> Air<AB> for StepFlagsAir<N> {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let (local, next) = (main.row_slice(0), main.row_slice(1));
        let local: &StepFlagsCols<AB::Var, N> = (*local).borrow();
        let next: &StepFlagsCols<AB::Var, N> = (*next).borrow();

        // Initially, the first step flag should be 1 while the others should be 0.
        builder.when_first_row().assert_one(local.flags[0]);
        for i in 1..N {
            builder.when_first_row().assert_zero(local.flags[i]);
        }
        for i in 0..N {
            let current_flag = local.flags[i];
            let next_flag = next.flags[(i + 1) % N];
            builder.assert_eq(next_flag, current_flag);
        }
    }
}
