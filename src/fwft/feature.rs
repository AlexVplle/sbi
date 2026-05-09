#[repr(u32)]
pub enum FwftFeature {
    MisalignedExcDeleg  = 0,
    LandingPad          = 1,
    ShadowStack         = 2,
    DoubleTrap          = 3,
    PteAdHwUpdating     = 4,
    PointerMaskingPmlen = 5,
}
