pub mod fw;
pub mod hw;
pub mod hw_cache;
pub mod hw_raw;
pub mod hw_raw_v2;

pub struct EventIdx(pub u32);

impl EventIdx {
    pub fn new(event_type: u32, event_code: u32) -> Self {
        Self((event_type & 0xF) << 16 | (event_code & 0xFFFF))
    }
}
