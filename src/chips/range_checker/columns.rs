use p3_derive::Columnar;

#[derive(Default, Columnar)]
pub struct RangeCols<T> {
    pub mult: T,
}

#[derive(Default, Columnar)]
pub struct RangePreprocessedCols<T> {
    pub counter: T,
}
