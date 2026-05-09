pub mod reset_reason;
pub mod reset_type;

use super::{SbiRet, eid, sbi_call_2};

const FID_SYSTEM_RESET: usize = 0;

pub fn system_reset(reset_type: u32, reset_reason: u32) -> ! {
    unsafe { sbi_call_2(eid::SRST, FID_SYSTEM_RESET, reset_type as usize, reset_reason as usize) };
    loop {}
}
