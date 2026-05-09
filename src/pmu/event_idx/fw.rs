use super::EventIdx;

#[repr(u32)]
pub enum FwEvent {
    MisalignedLoad = 0,
    MisalignedStore = 1,
    AccessLoad = 2,
    AccessStore = 3,
    IllegalInsn = 4,
    SetTimer = 5,
    IpiSent = 6,
    IpiRcvd = 7,
    FenceISent = 8,
    FenceIRcvd = 9,
    SfenceVmaSent = 10,
    SfenceVmaRcvd = 11,
    SfenceVmaAsidSent = 12,
    SfenceVmaAsidRcvd = 13,
    HfenceGvmaSent = 14,
    HfenceGvmaRcvd = 15,
    HfenceGvmaVmidSent = 16,
    HfenceGvmaVmidRcvd = 17,
    HfenceVvmaSent = 18,
    HfenceVvmaRcvd = 19,
    HfenceVvmaAsidSent = 20,
    HfenceVvmaAsidRcvd = 21,
}

impl From<FwEvent> for EventIdx {
    fn from(e: FwEvent) -> Self {
        EventIdx((0xF << 16) | e as u32)
    }
}
