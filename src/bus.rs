use p3_derive::Bus;

#[derive(Bus, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum KeccakMachineBus {
    KeccakPermuteInput = 0,
    KeccakPermuteOutput = 1,
    KeccakSpongeInput = 2,
    KeccakSpongeOutput = 3,
    XorInput = 4,
    XorOutput = 5,
    // Range8 = 6,
    // Memory = 7,
}
