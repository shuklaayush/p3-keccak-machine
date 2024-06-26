mod air;
pub mod columns;
mod interaction;
pub mod trace;
pub mod util;

#[derive(Default, Clone, Debug)]
pub struct KeccakSpongeChip {
    pub bus_input: usize,
    pub bus_output: usize,

    pub bus_xor_input: usize,
    pub bus_xor_output: usize,

    pub bus_permute_input: usize,
    pub bus_permute_output: usize,
}

#[cfg(feature = "air-logger")]
impl p3_air_util::AirLogger for KeccakSpongeChip {
    fn main_headers(&self) -> Vec<String> {
        self::columns::KeccakSpongeCols::<usize>::headers()
    }

    #[cfg(feature = "schema")]
    fn main_headers_and_types(&self) -> Vec<(String, String, core::ops::Range<usize>)> {
        self::columns::KeccakSpongeCols::<usize>::headers_and_types()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::prove_and_verify;

    use itertools::Itertools;
    use p3_uni_stark::VerificationError;
    use rand::random;
    use trace::KeccakSpongeOp;

    #[test]
    fn test_keccak_sponge_prove() -> Result<(), VerificationError> {
        const NUM_BYTES: usize = 400;

        let op = KeccakSpongeOp {
            timestamp: 0,
            addr: 0,
            input: (0..NUM_BYTES).map(|_| random()).collect_vec(),
        };
        let inputs = vec![op];
        let trace = KeccakSpongeChip::generate_trace(inputs);
        let chip = KeccakSpongeChip {
            ..Default::default()
        };

        prove_and_verify(&chip, trace, vec![])
    }
}
