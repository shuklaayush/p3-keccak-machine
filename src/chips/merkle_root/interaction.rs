use std::iter::once;

use p3_air::VirtualPairCol;
use p3_field::Field;
use p3_interaction::{BaseInteractionAir, Interaction, InteractionAir, InteractionAirBuilder, Rap};

use super::{columns::MerkleRootCols, MerkleRootChip};
use crate::chips::keccak_sponge::columns::KECCAK_RATE_BYTES;

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
                .into_iter()
                .map(|elem| VirtualPairCol::single_main(elem))
                .collect(),
            count: VirtualPairCol::single_main(col_map.is_real),
            argument_index: self.bus_hasher_output,
        }]
    }

    fn sends_from_indices(
        &self,
        _preprocessed_indices: &[usize],
        main_indices: &[usize],
    ) -> Vec<Interaction<F>> {
        let col_map = MerkleRootCols::<_, DEPTH, DIGEST_WIDTH>::from_slice(main_indices);
        vec![Interaction {
            fields: once(VirtualPairCol::constant(F::zero()))
                .chain(
                    col_map
                        .left_node
                        .into_iter()
                        .chain(col_map.right_node.into_iter())
                        .map(|elem| VirtualPairCol::single_main(elem))
                        // TODO: Don't send padding bytes
                        .chain((2 * DIGEST_WIDTH..KECCAK_RATE_BYTES).map(|i| {
                            VirtualPairCol::constant({
                                if i == 2 * DIGEST_WIDTH {
                                    F::one()
                                } else if i == KECCAK_RATE_BYTES - 1 {
                                    F::from_canonical_u8(0b10000000)
                                } else {
                                    F::zero()
                                }
                            })
                        })),
                )
                .collect(),
            count: VirtualPairCol::single_main(col_map.is_real),
            argument_index: self.bus_hasher_input,
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
