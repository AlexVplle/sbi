use super::EventIdx;

#[repr(u32)]
pub enum CacheId {
    L1D = 0,
    L1I = 1,
    Ll = 2,
    Dtlb = 3,
    Itlb = 4,
    Bpu = 5,
    Node = 6,
}

#[repr(u32)]
pub enum OpId {
    Read = 0,
    Write = 1,
    Prefetch = 2,
}

#[repr(u32)]
pub enum ResultId {
    Access = 0,
    Miss = 1,
}

pub fn new(cache_id: CacheId, op_id: OpId, result_id: ResultId) -> EventIdx {
    EventIdx((0x1 << 16) | ((cache_id as u32) << 3) | ((op_id as u32) << 1) | result_id as u32)
}
