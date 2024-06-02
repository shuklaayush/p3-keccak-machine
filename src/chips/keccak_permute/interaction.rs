use p3_air::VirtualPairCol;
use p3_field::Field;
use p3_interaction::{Interaction, InteractionAir, InteractionAirBuilder, Rap};

use super::{columns::KeccakPermuteCols, KeccakPermuteChip, NUM_U64_HASH_ELEMS};
use crate::airs::keccak::U64_LIMBS;

impl<F: Field> InteractionAir<F> for KeccakPermuteChip {
    fn receives(&self) -> Vec<Interaction<F>> {
        let col_map = KeccakPermuteCols::<F>::col_map();
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

    fn sends(&self) -> Vec<Interaction<F>> {
        let col_map = KeccakPermuteCols::<F>::col_map();
        vec![
            Interaction {
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
                count: VirtualPairCol::single_main(col_map.is_real_output_full),
                argument_index: self.bus_output_full,
            },
            Interaction {
                fields: (0..NUM_U64_HASH_ELEMS)
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
                count: VirtualPairCol::single_main(col_map.is_real_output_digest),
                argument_index: self.bus_output_digest,
            },
        ]
    }
}

impl<AB: InteractionAirBuilder> Rap<AB> for KeccakPermuteChip {}
