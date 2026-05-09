use super::EventIdx;

#[repr(u32)]
pub enum HwEvent {
    NoEvent = 0,
    CpuCycles = 1,
    Instructions = 2,
    CacheReferences = 3,
    CacheMisses = 4,
    BranchInstructions = 5,
    BranchMisses = 6,
    BusCycles = 7,
    StalledCyclesFrontend = 8,
    StalledCyclesBackend = 9,
    RefCpuCycles = 10,
}

impl From<HwEvent> for EventIdx {
    fn from(e: HwEvent) -> Self {
        EventIdx((0x0 << 16) | e as u32)
    }
}
