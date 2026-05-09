use super::{SbiRet, eid, sbi_call_1};

const FID_SET_TIMER: usize = 0;

pub fn set_timer(stime_value: u64) -> SbiRet {
    unsafe { sbi_call_1(eid::TIMER, FID_SET_TIMER, stime_value as usize) }
}
