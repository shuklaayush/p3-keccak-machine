use p3_derive::Bus;

#[derive(Bus, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum KeccakMachineBus {
    KeccakPermuteInputBus = 0,
    KeccakPermuteOutputFullBus = 1,
    KeccakPermuteOutputDigestBus = 2,
    Range8Bus = 3,
    XorInputBus = 4,
    XorOutputBus = 5,
    MemoryBus = 6,
}
