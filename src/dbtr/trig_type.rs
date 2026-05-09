#[repr(usize)]
pub enum TrigType {
    MControl  = 2,
    ICount    = 3,
    ITrigger  = 4,
    ETrigger  = 5,
    MControl6 = 6,
    Disabled  = 15,
}
