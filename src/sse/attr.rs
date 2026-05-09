#[repr(u32)]
pub enum SseAttr {
    Status           = 0,
    Priority         = 1,
    Config           = 2,
    PreferredHart    = 3,
    EntryPc          = 4,
    EntryArg         = 5,
    ScratchLo        = 6,
    ScratchHi        = 7,
    Cpe              = 8,
    InterruptNum     = 9,
    SeiImpdefData    = 10,
}
