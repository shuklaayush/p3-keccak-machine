use p3_derive::Columnar;

#[repr(C)]
#[derive(Columnar)]
pub struct StepFlagsCols<T, const N: usize> {
    pub flags: [T; N],
}
