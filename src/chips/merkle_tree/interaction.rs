use p3_air::VirtualPairCol;
use p3_field::Field;
use p3_interaction::{Interaction, InteractionAir, InteractionAirBuilder, Rap};

use super::{columns::MerkleTreeCols, MerkleRootChip};

impl<F: Field> InteractionAir<F> for MerkleRootChip {
    fn sends(&self) -> Vec<Interaction<F>> {
        let col_map = MerkleTreeCols::<F>::col_map();
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

    fn receives(&self) -> Vec<Interaction<F>> {
        let col_map = MerkleTreeCols::<F>::col_map();
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
}

impl<AB: InteractionAirBuilder> Rap<AB> for MerkleRootChip {}
