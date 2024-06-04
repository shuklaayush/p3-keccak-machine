use p3_air::VirtualPairCol;
use p3_field::Field;
use p3_interaction::{BaseInteractionAir, Interaction, InteractionAir, InteractionAirBuilder, Rap};

use super::{columns::MerkleRootCols, MerkleRootChip};

impl<F: Field> BaseInteractionAir<F> for MerkleRootChip {
    fn receives_from_indices(
        &self,
        _preprocessed_indices: &[usize],
        main_indices: &[usize],
    ) -> Vec<Interaction<F>> {
        let col_map = MerkleRootCols::from_slice(main_indices);
        vec![Interaction {
            fields: col_map
                .output
                .into_iter()
                .flatten()
                .map(VirtualPairCol::single_main)
                .collect(),
            count: VirtualPairCol::single_main(col_map.is_real),
            argument_index: self.bus_output,
        }]
    }

    fn sends_from_indices(
        &self,
        _preprocessed_indices: &[usize],
        main_indices: &[usize],
    ) -> Vec<Interaction<F>> {
        let col_map = MerkleRootCols::from_slice(main_indices);
        vec![Interaction {
            fields: col_map
                .left_node
                .into_iter()
                .chain(col_map.right_node)
                .flatten()
                .map(VirtualPairCol::single_main)
                .collect(),
            count: VirtualPairCol::single_main(col_map.is_real),
            argument_index: self.bus_input,
        }]
    }
}

impl<F: Field> InteractionAir<F> for MerkleRootChip {
    fn receives(&self) -> Vec<Interaction<F>> {
        let col_map = MerkleRootCols::<F>::col_map();
        self.receives_from_main_indices(col_map.as_slice())
    }

    fn sends(&self) -> Vec<Interaction<F>> {
        let col_map = MerkleRootCols::<F>::col_map();
        self.sends_from_main_indices(col_map.as_slice())
    }
}

impl<AB: InteractionAirBuilder> Rap<AB> for MerkleRootChip {}
