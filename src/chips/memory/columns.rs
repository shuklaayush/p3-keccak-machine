use p3_derive::{AirColumns, AlignedBorrow};

#[repr(C)]
#[derive(AlignedBorrow, AirColumns)]
pub struct MemoryCols<T> {
    pub addr: T,

    pub timestamp: T,

    pub value: T,

    pub is_read: T,

    pub is_write: T,

    // TODO: Do I need a column for this?
    pub addr_unchanged: T,

    /// Either addr' - addr - 1 (if address changed), or timestamp' - timestamp (if address is not changed)
    // No -1 in timestamp because can read and write in same cycle
    pub diff_limb_lo: T,
    pub diff_limb_md: T,
    pub diff_limb_hi: T,
}
