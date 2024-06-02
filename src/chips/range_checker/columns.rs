use p3_derive::Columns;

#[derive(Default, Columns)]
pub struct RangeCols<T> {
    pub mult: T,
}

#[derive(Default, Columns)]
pub struct RangePreprocessedCols<T> {
    pub counter: T,
}
