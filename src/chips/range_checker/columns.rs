use p3_derive::{AirColumns, AlignedBorrow};

#[derive(Default, AlignedBorrow, AirColumns)]
pub struct RangeCols<T> {
    pub mult: T,
}

#[derive(Default, AlignedBorrow, AirColumns)]
pub struct RangePreprocessedCols<T> {
    pub counter: T,
}
