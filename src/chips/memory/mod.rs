mod air;
mod columns;
mod interaction;
mod trace;

#[derive(Default, Clone, Debug)]
pub struct MemoryChip {
    pub bus_memory: usize,
    pub bus_range_8: usize,
}

#[cfg(feature = "air-logger")]
impl p3_air_util::AirLogger for MemoryChip {
    fn main_headers(&self) -> Vec<String> {
        self::columns::MemoryCols::<usize>::headers()
    }

    #[cfg(feature = "schema")]
    fn main_headers_and_types(&self) -> Vec<(String, String, core::ops::Range<usize>)> {
        self::columns::MemoryCols::<usize>::headers_and_types()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::prove_and_verify;

    use itertools::Itertools;
    use p3_uni_stark::VerificationError;
    use rand::random;
    use trace::{MemoryOp, OperationKind};

    #[test]
    fn test_memory_prove() -> Result<(), VerificationError> {
        const NUM_BYTES: usize = 400;

        let bytes = (0..NUM_BYTES).map(|_| random()).collect_vec();
        let operations = bytes
            .into_iter()
            .enumerate()
            .map(|(i, b)| MemoryOp {
                addr: i as u32,
                timestamp: i as u32,
                value: b,
                kind: OperationKind::Read,
            })
            .collect_vec();
        let trace = MemoryChip::generate_trace(operations);
        let chip = MemoryChip {
            ..Default::default()
        };

        prove_and_verify(&chip, trace, vec![])
    }
}
