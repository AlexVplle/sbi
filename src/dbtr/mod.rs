pub mod trig_type;

use super::{SbiRet, eid, sbi_call_1, sbi_call_2, sbi_call_3, sbi_call_4, sbi_call_5};

const FID_NUM_TRIGGERS: usize = 0;
const FID_SETUP_SHMEM: usize = 1;
const FID_READ_TRIG: usize = 2;
const FID_INSTALL_TRIG: usize = 3;
const FID_UPDATE_TRIG: usize = 4;
const FID_UNINSTALL_TRIG: usize = 5;
const FID_ENABLE_TRIG: usize = 6;
const FID_DISABLE_TRIG: usize = 7;

pub fn num_triggers(trig_tdata1: usize) -> SbiRet {
    unsafe { sbi_call_1(eid::DBTR, FID_NUM_TRIGGERS, trig_tdata1) }
}

pub fn setup_shmem(shmem_phys_lo: usize, shmem_phys_hi: usize, flags: usize) -> SbiRet {
    unsafe { sbi_call_3(eid::DBTR, FID_SETUP_SHMEM, shmem_phys_lo, shmem_phys_hi, flags) }
}

pub fn read_trig(trig_idx: usize) -> SbiRet {
    unsafe { sbi_call_1(eid::DBTR, FID_READ_TRIG, trig_idx) }
}

pub fn install_trig(trig_count: usize, tdata1: usize, tdata2: usize, tdata3: usize, flags: usize) -> SbiRet {
    unsafe { sbi_call_5(eid::DBTR, FID_INSTALL_TRIG, trig_count, tdata1, tdata2, tdata3, flags) }
}

pub fn update_trig(trig_idx: usize, tdata1: usize, tdata2: usize, tdata3: usize) -> SbiRet {
    unsafe { sbi_call_4(eid::DBTR, FID_UPDATE_TRIG, trig_idx, tdata1, tdata2, tdata3) }
}

pub fn uninstall_trig(trig_idx: usize, trig_count: usize) -> SbiRet {
    unsafe { sbi_call_2(eid::DBTR, FID_UNINSTALL_TRIG, trig_idx, trig_count) }
}

pub fn enable_trig(trig_idx: usize, trig_count: usize) -> SbiRet {
    unsafe { sbi_call_2(eid::DBTR, FID_ENABLE_TRIG, trig_idx, trig_count) }
}

pub fn disable_trig(trig_idx: usize, trig_count: usize) -> SbiRet {
    unsafe { sbi_call_2(eid::DBTR, FID_DISABLE_TRIG, trig_idx, trig_count) }
}
