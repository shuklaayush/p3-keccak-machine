mod air;
mod columns;
mod constants;
mod generation;
mod logic;
mod round_flags;

pub use air::*;
pub use columns::*;
pub use constants::*;
pub use generation::*;

pub const NUM_ROUNDS: usize = 24;
const BITS_PER_LIMB: usize = 16;
pub const U64_LIMBS: usize = 64 / BITS_PER_LIMB;
const RATE_BITS: usize = 1088;
const RATE_LIMBS: usize = RATE_BITS / BITS_PER_LIMB;
