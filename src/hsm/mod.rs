pub mod hart_state;
pub mod suspend_type;

use hart_state::HartState;
use super::{SbiRet, eid, sbi_call_0, sbi_call_1, sbi_call_3};

const FID_HART_START: usize = 0;
const FID_HART_STOP: usize = 1;
const FID_HART_GET_STATUS: usize = 2;
const FID_HART_SUSPEND: usize = 3;

pub fn hart_start(hartid: usize, start_addr: usize, opaque: usize) -> SbiRet {
    unsafe { sbi_call_3(eid::HSM, FID_HART_START, hartid, start_addr, opaque) }
}

pub fn hart_stop() -> SbiRet {
    unsafe { sbi_call_0(eid::HSM, FID_HART_STOP) }
}

pub fn hart_get_status(hartid: usize) -> Result<HartState, SbiRet> {
    let ret: SbiRet = unsafe { sbi_call_1(eid::HSM, FID_HART_GET_STATUS, hartid) };
    match ret.value {
        0 => Ok(HartState::Started),
        1 => Ok(HartState::Stopped),
        2 => Ok(HartState::StartPending),
        3 => Ok(HartState::StopPending),
        4 => Ok(HartState::Suspended),
        5 => Ok(HartState::SuspendPending),
        6 => Ok(HartState::ResumePending),
        _ => Err(ret),
    }
}

pub fn hart_suspend(suspend_type: u32, resume_addr: usize, opaque: usize) -> SbiRet {
    unsafe { sbi_call_3(eid::HSM, FID_HART_SUSPEND, suspend_type as usize, resume_addr, opaque) }
}
