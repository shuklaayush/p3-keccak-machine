use alloc::vec;
use alloc::vec::Vec;

use p3_air::VirtualPairCol;
use p3_field::Field;
use p3_interaction::{Interaction, InteractionAir, InteractionAirBuilder, Rap};

use super::{
    columns::{RangeCols, RangePreprocessedCols},
    RangeCheckerChip,
};

impl<const MAX: u32, F: Field> InteractionAir<F> for RangeCheckerChip<MAX> {
    fn receives(&self) -> Vec<Interaction<F>> {
        let preprocessed_col_map = RangePreprocessedCols::<F>::col_map();
        let main_col_map = RangeCols::<F>::col_map();

        vec![Interaction {
            fields: vec![VirtualPairCol::single_preprocessed(
                preprocessed_col_map.counter,
            )],
            count: VirtualPairCol::single_main(main_col_map.mult),
            argument_index: self.bus_range_8,
        }]
    }
}

impl<const MAX: u32, AB: InteractionAirBuilder> Rap<AB> for RangeCheckerChip<MAX> {
    fn preprocessed_width(&self) -> usize {
        1
    }
}
