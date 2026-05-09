#[repr(usize)]
pub enum NaclFeature {
    SyncCsr      = 0,
    SyncHfence   = 1,
    SyncSret     = 2,
    AutoSwapCsr  = 3,
}
