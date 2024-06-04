use p3_air::VirtualPairCol;
use p3_field::Field;
use p3_interaction::{BaseInteractionAir, Interaction, InteractionAir, InteractionAirBuilder, Rap};

use super::{columns::MemoryCols, MemoryChip};

impl<F: Field> BaseInteractionAir<F> for MemoryChip {
    fn receives_from_indices(
        &self,
        _preprocessed_indices: &[usize],
        main_indices: &[usize],
    ) -> Vec<Interaction<F>> {
        let col_map = MemoryCols::from_usize_slice(main_indices);

        vec![Interaction {
            fields: vec![
                VirtualPairCol::single_main(col_map.timestamp),
                VirtualPairCol::single_main(col_map.addr),
                VirtualPairCol::single_main(col_map.value),
            ],
            count: VirtualPairCol::single_main(col_map.is_write),
            argument_index: self.bus_memory,
        }]
    }

    fn sends_from_indices(
        &self,
        _preprocessed_indices: &[usize],
        main_indices: &[usize],
    ) -> Vec<Interaction<F>> {
        let col_map = MemoryCols::from_usize_slice(main_indices);

        vec![
            // TODO: Combine with is_write?
            Interaction {
                fields: vec![
                    VirtualPairCol::single_main(col_map.timestamp),
                    VirtualPairCol::single_main(col_map.addr),
                    VirtualPairCol::single_main(col_map.value),
                ],
                count: VirtualPairCol::single_main(col_map.is_read),
                argument_index: self.bus_memory,
            },
            Interaction {
                fields: vec![VirtualPairCol::single_main(col_map.diff_limb_lo)],
                count: VirtualPairCol::sum_main(vec![col_map.is_read, col_map.is_write]),
                argument_index: self.bus_range_8,
            },
            Interaction {
                fields: vec![VirtualPairCol::single_main(col_map.diff_limb_md)],
                count: VirtualPairCol::sum_main(vec![col_map.is_read, col_map.is_write]),
                argument_index: self.bus_range_8,
            },
            Interaction {
                fields: vec![VirtualPairCol::single_main(col_map.diff_limb_hi)],
                count: VirtualPairCol::sum_main(vec![col_map.is_read, col_map.is_write]),
                argument_index: self.bus_range_8,
            },
        ]
    }
}

impl<F: Field> InteractionAir<F> for MemoryChip {
    fn receives(&self) -> Vec<Interaction<F>> {
        let col_map = MemoryCols::<F>::col_map();
        self.receives_from_main_indices(col_map.as_usize_slice())
    }

    fn sends(&self) -> Vec<Interaction<F>> {
        let col_map = MemoryCols::<F>::col_map();
        self.sends_from_main_indices(col_map.as_usize_slice())
    }
}

impl<AB: InteractionAirBuilder> Rap<AB> for MemoryChip {}
