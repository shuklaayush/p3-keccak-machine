use alloc::vec;
use alloc::vec::Vec;

use p3_air::VirtualPairCol;
use p3_field::Field;
use p3_interaction::{BaseInteractionAir, Interaction, InteractionAir, InteractionAirBuilder, Rap};

use super::{
    columns::{RangeCols, RangePreprocessedCols},
    RangeCheckerChip,
};

impl<const MAX: u32, F: Field> BaseInteractionAir<F> for RangeCheckerChip<MAX> {
    fn receives_from_indices(
        &self,
        preprocessed_indices: &[usize],
        main_indices: &[usize],
    ) -> Vec<Interaction<F>> {
        let preprocessed_col_map = RangePreprocessedCols::from_slice(preprocessed_indices);
        let main_col_map = RangeCols::from_slice(main_indices);

        vec![Interaction {
            fields: vec![VirtualPairCol::single_preprocessed(
                preprocessed_col_map.counter,
            )],
            count: VirtualPairCol::single_main(main_col_map.mult),
            argument_index: self.bus_range_8,
        }]
    }
}

impl<const MAX: u32, F: Field> InteractionAir<F> for RangeCheckerChip<MAX> {
    fn receives(&self) -> Vec<Interaction<F>> {
        let preprocessed_col_map = RangePreprocessedCols::<F>::col_map();
        let main_col_map = RangeCols::<F>::col_map();

        self.receives_from_indices(preprocessed_col_map.as_slice(), main_col_map.as_slice())
    }
}

impl<const MAX: u32, AB: InteractionAirBuilder> Rap<AB> for RangeCheckerChip<MAX> {
    fn preprocessed_width(&self) -> usize {
        RangePreprocessedCols::<AB::F>::num_cols()
    }
}
