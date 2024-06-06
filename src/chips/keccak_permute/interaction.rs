use p3_air::VirtualPairCol;
use p3_field::Field;
use p3_interaction::{BaseInteractionAir, Interaction, InteractionAir, InteractionAirBuilder, Rap};

use super::{columns::KeccakPermuteCols, KeccakPermuteChip};
use crate::airs::keccak::U64_LIMBS;

impl<F: Field> BaseInteractionAir<F> for KeccakPermuteChip {
    fn receives_from_indices(
        &self,
        _preprocessed_indices: &[usize],
        main_indices: &[usize],
    ) -> Vec<Interaction<F>> {
        let col_map = KeccakPermuteCols::from_slice(main_indices);

        vec![Interaction {
            fields: col_map
                .keccak
                .preimage
                .into_iter()
                .flatten()
                .flatten()
                .map(VirtualPairCol::single_main)
                .collect(),
            count: VirtualPairCol::single_main(col_map.is_real_input),
            argument_index: self.bus_input,
        }]
    }

    fn sends_from_indices(
        &self,
        _preprocessed_indices: &[usize],
        main_indices: &[usize],
    ) -> Vec<Interaction<F>> {
        let col_map = KeccakPermuteCols::from_slice(main_indices);

        vec![Interaction {
            fields: (0..25)
                .flat_map(|i| {
                    (0..U64_LIMBS)
                        .map(|limb| {
                            let y = i / 5;
                            let x = i % 5;
                            col_map.keccak.a_prime_prime_prime(y, x, limb)
                        })
                        .collect::<Vec<_>>()
                })
                .map(VirtualPairCol::single_main)
                .collect(),
            count: VirtualPairCol::single_main(col_map.is_real_output),
            argument_index: self.bus_output,
        }]
    }
}

impl<F: Field> InteractionAir<F> for KeccakPermuteChip {
    fn receives(&self) -> Vec<Interaction<F>> {
        let col_map = KeccakPermuteCols::<F>::col_map();
        self.receives_from_main_indices(col_map.as_slice())
    }

    fn sends(&self) -> Vec<Interaction<F>> {
        let col_map = KeccakPermuteCols::<F>::col_map();
        self.sends_from_main_indices(col_map.as_slice())
    }
}

impl<AB: InteractionAirBuilder> Rap<AB> for KeccakPermuteChip {}
