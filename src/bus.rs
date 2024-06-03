use p3_derive::Bus;

#[derive(Bus)]
pub enum KeccakMachineBus {
    KeccakPermuteInput = 0,
    KeccakPermuteOutputFull = 1,
    KeccakPermuteOutputDigest = 2,
    Range8 = 3,
    XorInput = 4,
    XorOutput = 5,
    Memory = 6,
}
