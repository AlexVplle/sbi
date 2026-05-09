use super::EventIdx;

pub fn new(event_code: u32) -> EventIdx {
    EventIdx((0x3 << 16) | (event_code & 0xFFFF))
}
