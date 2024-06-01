use p3_derive::AirColumns;

#[derive(Default, AirColumns)]
pub struct RangeCols<T> {
    pub mult: T,
}

#[derive(Default, AirColumns)]
pub struct RangePreprocessedCols<T> {
    pub counter: T,
}
