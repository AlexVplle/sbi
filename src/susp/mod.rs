pub mod sleep_type;

use sleep_type::SleepType;
use super::{SbiRet, eid, sbi_call_3};

const FID_SYSTEM_SUSPEND: usize = 0;

pub fn system_suspend(sleep_type: SleepType, resume_addr: usize, opaque: usize) -> SbiRet {
    unsafe { sbi_call_3(eid::SUSP, FID_SYSTEM_SUSPEND, sleep_type as usize, resume_addr, opaque) }
}
