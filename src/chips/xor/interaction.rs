use alloc::vec;
use alloc::vec::Vec;

use p3_air::VirtualPairCol;
use p3_field::Field;
use p3_interaction::{BaseInteractionAir, Interaction, InteractionAir, InteractionAirBuilder, Rap};

use super::{columns::XorCols, XorChip};

impl<F: Field> BaseInteractionAir<F> for XorChip {
    fn receives_from_indices(
        &self,
        _preprocessed_indices: &[usize],
        main_indices: &[usize],
    ) -> Vec<Interaction<F>> {
        let col_map = XorCols::from_usize_slice(main_indices);
        let vc1 = {
            let column_weights = col_map
                .input1
                .into_iter()
                .enumerate()
                .map(|(i, c)| (c, F::from_canonical_usize(1 << (8 * i))))
                .collect();
            VirtualPairCol::new_main(column_weights, F::zero())
        };
        let vc2 = {
            let column_weights = col_map
                .input2
                .into_iter()
                .enumerate()
                .map(|(i, c)| (c, F::from_canonical_usize(1 << (8 * i))))
                .collect();
            VirtualPairCol::new_main(column_weights, F::zero())
        };
        vec![Interaction {
            fields: vec![vc1, vc2],
            count: VirtualPairCol::single_main(col_map.is_real),
            argument_index: self.bus_input,
        }]
    }

    fn sends_from_indices(
        &self,
        _preprocessed_indices: &[usize],
        main_indices: &[usize],
    ) -> Vec<Interaction<F>> {
        let col_map = XorCols::from_usize_slice(main_indices);
        let column_weights = col_map
            .output
            .into_iter()
            .enumerate()
            .map(|(i, c)| (c, F::from_canonical_usize(1 << (8 * i))))
            .collect();
        vec![Interaction {
            fields: vec![VirtualPairCol::new_main(column_weights, F::zero())],
            count: VirtualPairCol::single_main(col_map.is_real),
            argument_index: self.bus_output,
        }]
    }
}

impl<F: Field> InteractionAir<F> for XorChip {
    fn receives(&self) -> Vec<Interaction<F>> {
        let col_map = XorCols::<F>::col_map();
        self.receives_from_main_indices(col_map.as_usize_slice())
    }

    fn sends(&self) -> Vec<Interaction<F>> {
        let col_map = XorCols::<F>::col_map();
        self.sends_from_main_indices(col_map.as_usize_slice())
    }
}

impl<AB: InteractionAirBuilder> Rap<AB> for XorChip {}
