mod air;
mod columns;
mod interaction;
mod trace;

#[derive(Default, Clone, Debug)]
pub struct RangeCheckerChip<const MAX: u32> {
    pub bus_range_8: usize,
}

#[cfg(feature = "air-logger")]
impl<const MAX: u32> p3_air_util::AirLogger for RangeCheckerChip<MAX> {
    fn preprocessed_headers(&self) -> Vec<String> {
        self::columns::RangePreprocessedCols::<usize>::headers()
    }

    fn main_headers(&self) -> Vec<String> {
        self::columns::RangeCols::<usize>::headers()
    }

    fn preprocessed_headers_and_types(&self) -> Vec<(String, String, core::ops::Range<usize>)> {
        self::columns::RangePreprocessedCols::<usize>::headers_and_types()
    }

    fn main_headers_and_types(&self) -> Vec<(String, String, core::ops::Range<usize>)> {
        self::columns::RangeCols::<usize>::headers_and_types()
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
