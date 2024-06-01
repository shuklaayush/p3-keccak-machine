use p3_air::VirtualPairCol;
use p3_field::Field;
use p3_interaction::{Interaction, InteractionAir, InteractionAirBuilder, Rap};

use super::{columns::MemoryCols, MemoryChip};

impl<F: Field> InteractionAir<F> for MemoryChip {
    fn receives(&self) -> Vec<Interaction<F>> {
        let col_map = MemoryCols::<F>::col_map();
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

    fn sends(&self) -> Vec<Interaction<F>> {
        let col_map = MemoryCols::<F>::col_map();
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

impl<AB: InteractionAirBuilder> Rap<AB> for MemoryChip {}
