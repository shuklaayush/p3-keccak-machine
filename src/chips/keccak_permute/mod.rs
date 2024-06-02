mod air;
mod columns;
mod interaction;
pub mod trace;

#[cfg(feature = "trace-writer")]
use p3_air_util::TraceWriter;
#[cfg(feature = "trace-writer")]
use p3_field::{ExtensionField, Field};

pub const NUM_U64_HASH_ELEMS: usize = 4;

/// Assumes the field size is at least 16 bits.
#[derive(Clone, Debug)]
pub struct KeccakPermuteChip {
    pub bus_input: usize,

    pub bus_output_full: usize,
    pub bus_output_digest: usize,
}

#[cfg(feature = "trace-writer")]
impl<F: Field, EF: ExtensionField<F>> TraceWriter<F, EF> for KeccakPermuteChip {
    fn headers(&self) -> Vec<String> {
        self::columns::KeccakPermuteCols::<F>::headers()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::prove_and_verify;

    use itertools::Itertools;
    use p3_uni_stark::VerificationError;
    use rand::random;
    use trace::{KeccakPermuteOp, KeccakPermuteOpType};

    #[test]
    fn test_keccak_permute_prove() -> Result<(), VerificationError> {
        const NUM_PERMS: usize = 10;

        let chip = KeccakPermuteChip {
            bus_input: 0,
            bus_output_full: 0,
            bus_output_digest: 0,
        };
        let inputs = (0..NUM_PERMS)
            .map(|_| KeccakPermuteOp {
                input: random(),
                op_type: KeccakPermuteOpType::Full,
            })
            .collect_vec();
        let trace = KeccakPermuteChip::generate_trace(inputs);

        prove_and_verify(&chip, trace, vec![])
    }
}
