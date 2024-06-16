use p3_derive::Columnar;

#[derive(Debug, Columnar)]
#[repr(C)]
pub struct PoseidonCols<T, const WIDTH: usize, const N_ROUNDS: usize> {
    /// The `i`th value is set to 1 if we are in the `i`th round, otherwise 0.
    pub round_flags: [T; N_ROUNDS],

    /// Set to 1 if we are currently in a partial round, otherwise 0.
    pub partial_round: T,

    pub start_of_round: [T; WIDTH],
    pub after_constants: [T; WIDTH],
    pub after_sbox: [T; WIDTH],
    pub after_mds: [T; WIDTH],
}
