mod air;
mod columns;
mod interaction;
mod trace;

#[cfg(feature = "trace-writer")]
use p3_air_util::TraceWriter;
#[cfg(feature = "trace-writer")]
use p3_field::{ExtensionField, Field};

#[derive(Default, Clone, Debug)]
pub struct RangeCheckerChip<const MAX: u32> {
    pub bus_range_8: usize,
}

#[cfg(feature = "trace-writer")]
impl<const MAX: u32, F: Field, EF: ExtensionField<F>> TraceWriter<F, EF> for RangeCheckerChip<MAX> {
    fn preprocessed_headers(&self) -> Vec<String> {
        self::columns::RangePreprocessedCols::<F>::headers()
    }

    fn headers(&self) -> Vec<String> {
        self::columns::RangeCols::<F>::headers()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::prove_and_verify;

    use p3_uni_stark::VerificationError;
    use rand::random;
    use std::collections::BTreeMap;

    #[test]
    fn test_range_prove() -> Result<(), VerificationError> {
        const NUM: usize = 400;

        let mut count = BTreeMap::new();
        for _ in 0..NUM {
            count
                .entry(random::<u8>() as u32)
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }
        let trace = RangeCheckerChip::<256>::generate_trace(count);
        let chip = RangeCheckerChip::<256> {
            ..Default::default()
        };

        prove_and_verify(&chip, trace, vec![])
    }
}
