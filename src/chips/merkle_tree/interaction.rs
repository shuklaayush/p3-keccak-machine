use p3_air::VirtualPairCol;
use p3_field::Field;
use p3_interaction::{BaseInteractionAir, Interaction, InteractionAir, InteractionAirBuilder, Rap};

use super::{columns::MerkleRootCols, MerkleRootChip};

impl<F, const DEPTH: usize, const DIGEST_WIDTH: usize> BaseInteractionAir<F>
    for MerkleRootChip<DEPTH, DIGEST_WIDTH>
where
    F: Field,
{
    fn receives_from_indices(
        &self,
        _preprocessed_indices: &[usize],
        main_indices: &[usize],
    ) -> Vec<Interaction<F>> {
        let col_map = MerkleRootCols::<_, DEPTH, DIGEST_WIDTH>::from_slice(main_indices);
        vec![Interaction {
            fields: col_map
                .output
                .chunks_exact(2)
                .map(|limbs| {
                    VirtualPairCol::new_main(
                        vec![
                            (limbs[0], F::one()),
                            (limbs[1], F::from_canonical_usize(1 << 8)),
                        ],
                        F::zero(),
                    )
                })
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
        let col_map = MerkleRootCols::<_, DEPTH, DIGEST_WIDTH>::from_slice(main_indices);
        vec![Interaction {
            fields: col_map
                .left_node
                .chunks_exact(2)
                .chain(col_map.right_node.chunks(2))
                .map(|limbs| {
                    VirtualPairCol::new_main(
                        vec![
                            (limbs[0], F::one()),
                            (limbs[1], F::from_canonical_usize(1 << 8)),
                        ],
                        F::zero(),
                    )
                })
                .collect(),
            count: VirtualPairCol::single_main(col_map.is_real),
            argument_index: self.bus_input,
        }]
    }
}

impl<F, const DEPTH: usize, const DIGEST_WIDTH: usize> InteractionAir<F>
    for MerkleRootChip<DEPTH, DIGEST_WIDTH>
where
    F: Field,
{
    fn receives(&self) -> Vec<Interaction<F>> {
        let col_map = MerkleRootCols::<F, DEPTH, DIGEST_WIDTH>::col_map();
        self.receives_from_main_indices(col_map.as_slice())
    }

    fn sends(&self) -> Vec<Interaction<F>> {
        let col_map = MerkleRootCols::<F, DEPTH, DIGEST_WIDTH>::col_map();
        self.sends_from_main_indices(col_map.as_slice())
    }
}

impl<AB, const DEPTH: usize, const DIGEST_WIDTH: usize> Rap<AB>
    for MerkleRootChip<DEPTH, DIGEST_WIDTH>
where
    AB: InteractionAirBuilder,
{
}
