use std::iter::once;

use itertools::Itertools;
use p3_air::VirtualPairCol;
use p3_field::Field;
use p3_interaction::{BaseInteractionAir, Interaction, InteractionAir, InteractionAirBuilder, Rap};

use super::{
    columns::{KeccakSpongeCols, KECCAK_DIGEST_BYTES, KECCAK_RATE_BYTES},
    KeccakSpongeChip,
};

impl<F: Field> BaseInteractionAir<F> for KeccakSpongeChip {
    fn receives_from_indices(
        &self,
        _preprocessed_indices: &[usize],
        main_indices: &[usize],
    ) -> Vec<Interaction<F>> {
        let col_map = KeccakSpongeCols::from_slice(main_indices);

        let is_real = VirtualPairCol::sum_main(vec![
            col_map.is_padding_byte[KECCAK_RATE_BYTES - 1],
            col_map.is_full_input_block,
        ]);
        [
            // (0..KECCAK_RATE_BYTES)
            //     .map(|i| {
            //         let is_real = if i == KECCAK_RATE_BYTES - 1 {
            //             VirtualPairCol::single_main(col_map.is_full_input_block)
            //         } else {
            //             VirtualPairCol::new_main(
            //                 vec![
            //                     (col_map.is_full_input_block, F::one()),
            //                     (col_map.is_padding_byte[KECCAK_RATE_BYTES - 1], F::one()),
            //                     (col_map.is_padding_byte[i], -F::one()),
            //                 ],
            //                 F::zero(),
            //             )
            //         };
            //         Interaction {
            //             fields: vec![
            //                 VirtualPairCol::single_main(col_map.timestamp),
            //                 VirtualPairCol::new_main(
            //                     vec![
            //                         (col_map.base_addr, F::one()),
            //                         (col_map.already_absorbed_bytes, F::one()),
            //                     ],
            //                     F::from_canonical_usize(i),
            //                 ),
            //                 VirtualPairCol::single_main(col_map.block_bytes[i]),
            //             ],
            //             count: is_real,
            //             argument_index: self.bus_input,
            //         }
            //     })
            //     .collect_vec(),
            // TODO: Only send non padding bytes. Interaction field should be
            //       is_padding_byte[i] * block_bytes[i] but requires degree 2 fields
            vec![Interaction {
                fields: once(VirtualPairCol::single_main(col_map.is_full_input_block))
                    .chain(
                        (0..KECCAK_RATE_BYTES)
                            .map(|i| VirtualPairCol::single_main(col_map.block_bytes[i])),
                    )
                    .collect_vec(),
                count: is_real.clone(),
                argument_index: self.bus_input,
            }],
            col_map
                .xored_rate_u16s
                .into_iter()
                .map(|rate_limb| Interaction {
                    fields: vec![VirtualPairCol::single_main(rate_limb)],
                    count: is_real.clone(),
                    argument_index: self.bus_xor_output,
                })
                .collect_vec(),
            vec![Interaction {
                // We recover the 16-bit digest limbs from their corresponding bytes,
                // and then append them to the rest of the updated state limbs.
                fields: col_map
                    .updated_digest_state_bytes
                    .chunks(2)
                    .map(|cols| {
                        let column_weights = cols
                            .iter()
                            .enumerate()
                            .map(|(i, &c)| (c, F::from_canonical_usize(1 << (8 * i))))
                            .collect_vec();
                        VirtualPairCol::new_main(column_weights, F::zero())
                    })
                    .chain(
                        col_map
                            .partial_updated_state_u16s
                            .into_iter()
                            .map(VirtualPairCol::single_main),
                    )
                    .collect_vec(),
                count: is_real.clone(),
                argument_index: self.bus_permute_output,
            }],
        ]
        .concat()
    }

    fn sends_from_indices(
        &self,
        _preprocessed_indices: &[usize],
        main_indices: &[usize],
    ) -> Vec<Interaction<F>> {
        let col_map = KeccakSpongeCols::from_slice(main_indices);

        let is_real = VirtualPairCol::sum_main(vec![
            col_map.is_padding_byte[KECCAK_RATE_BYTES - 1],
            col_map.is_full_input_block,
        ]);
        [
            col_map
                .block_bytes
                .chunks(2)
                .zip(col_map.original_rate_u16s)
                .map(|(block_byte, rate_limb)| {
                    let vc1 = {
                        let column_weights = block_byte
                            .iter()
                            .enumerate()
                            .map(|(i, &c)| (c, F::from_canonical_usize(1 << (8 * i))))
                            .collect_vec();
                        VirtualPairCol::new_main(column_weights, F::zero())
                    };
                    let vc2 = VirtualPairCol::single_main(rate_limb);
                    Interaction {
                        fields: vec![vc1, vc2],
                        count: is_real.clone(),
                        argument_index: self.bus_xor_input,
                    }
                })
                .collect_vec(),
            vec![Interaction {
                fields: col_map
                    .xored_rate_u16s
                    .into_iter()
                    .chain(col_map.original_capacity_u16s)
                    .map(VirtualPairCol::single_main)
                    .collect(),
                count: is_real.clone(),
                argument_index: self.bus_permute_input,
            }],
            vec![Interaction {
                fields: (0..KECCAK_DIGEST_BYTES)
                    .map(|i| VirtualPairCol::single_main(col_map.updated_digest_state_bytes[i]))
                    .collect_vec(),
                count: is_real.clone(),
                argument_index: self.bus_output,
            }],
        ]
        .concat()
    }
}

impl<F: Field> InteractionAir<F> for KeccakSpongeChip {
    fn receives(&self) -> Vec<Interaction<F>> {
        let col_map = KeccakSpongeCols::<F>::col_map();
        self.receives_from_main_indices(col_map.as_slice())
    }

    fn sends(&self) -> Vec<Interaction<F>> {
        let col_map = KeccakSpongeCols::<F>::col_map();
        self.sends_from_main_indices(col_map.as_slice())
    }
}

impl<AB: InteractionAirBuilder> Rap<AB> for KeccakSpongeChip {}
