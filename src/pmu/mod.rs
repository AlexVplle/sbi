pub mod config_flags;
pub mod event_idx;
pub mod start_flags;
pub mod stop_flags;

use config_flags::ConfigFlags;
use event_idx::EventIdx;
use start_flags::StartFlags;
use stop_flags::StopFlags;
use super::{SbiRet, eid, sbi_call_0, sbi_call_1, sbi_call_3, sbi_call_4, sbi_call_5};

const FID_NUM_COUNTERS: usize = 0;
const FID_COUNTER_GET_INFO: usize = 1;
const FID_COUNTER_CONFIG_MATCHING: usize = 2;
const FID_COUNTER_START: usize = 3;
const FID_COUNTER_STOP: usize = 4;
const FID_COUNTER_FW_READ: usize = 5;
const FID_COUNTER_FW_READ_HI: usize = 6;
const FID_SNAPSHOT_SET_SHMEM: usize = 7;

pub fn num_counters() -> SbiRet {
    unsafe { sbi_call_0(eid::PMU, FID_NUM_COUNTERS) }
}

pub fn counter_get_info(counter_idx: usize) -> SbiRet {
    unsafe { sbi_call_1(eid::PMU, FID_COUNTER_GET_INFO, counter_idx) }
}

pub fn counter_config_matching(counter_idx_base: usize, counter_idx_mask: usize, config_flags: ConfigFlags, event_idx: EventIdx, event_data: u64) -> SbiRet {
    unsafe { sbi_call_5(eid::PMU, FID_COUNTER_CONFIG_MATCHING, counter_idx_base, counter_idx_mask, config_flags.bits(), event_idx.0 as usize, event_data as usize) }
}

pub fn counter_start(counter_idx_base: usize, counter_idx_mask: usize, start_flags: StartFlags, initial_value: u64) -> SbiRet {
    unsafe { sbi_call_4(eid::PMU, FID_COUNTER_START, counter_idx_base, counter_idx_mask, start_flags.bits(), initial_value as usize) }
}

pub fn counter_stop(counter_idx_base: usize, counter_idx_mask: usize, stop_flags: StopFlags) -> SbiRet {
    unsafe { sbi_call_3(eid::PMU, FID_COUNTER_STOP, counter_idx_base, counter_idx_mask, stop_flags.bits()) }
}

pub fn counter_fw_read(counter_idx: usize) -> SbiRet {
    unsafe { sbi_call_1(eid::PMU, FID_COUNTER_FW_READ, counter_idx) }
}

pub fn counter_fw_read_hi(counter_idx: usize) -> SbiRet {
    unsafe { sbi_call_1(eid::PMU, FID_COUNTER_FW_READ_HI, counter_idx) }
}

pub fn snapshot_set_shmem(shmem_phys_lo: usize, shmem_phys_hi: usize) -> SbiRet {
    unsafe { sbi_call_3(eid::PMU, FID_SNAPSHOT_SET_SHMEM, shmem_phys_lo, shmem_phys_hi, 0) }
}
