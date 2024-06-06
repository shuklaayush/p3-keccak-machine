use p3_derive::Columnar;

use crate::airs::step_flags::StepFlagsCols;

#[repr(C)]
#[derive(Columnar)]
pub struct MerkleRootCols<T, const DEPTH: usize, const DIGEST_WIDTH: usize> {
    pub is_real: T,

    pub step_flags: StepFlagsCols<T, DEPTH>,

    pub node: [T; DIGEST_WIDTH],

    pub sibling: [T; DIGEST_WIDTH],

    pub is_right_child: T,

    pub accumulated_index: T,

    pub left_node: [T; DIGEST_WIDTH],

    pub right_node: [T; DIGEST_WIDTH],

    pub output: [T; DIGEST_WIDTH],
}
