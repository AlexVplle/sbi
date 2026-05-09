pub mod feature;

use feature::NaclFeature;
use super::{SbiRet, eid, sbi_call_1, sbi_call_3};

const FID_PROBE_FEATURE: usize = 0;
const FID_SET_SHMEM: usize = 1;
const FID_SYNC_CSR: usize = 2;
const FID_SYNC_HFENCE: usize = 3;
const FID_SYNC_SRET: usize = 4;

pub fn probe_feature(feature: NaclFeature) -> SbiRet {
    unsafe { sbi_call_1(eid::NACL, FID_PROBE_FEATURE, feature as usize) }
}

pub fn set_shmem(shmem_phys_lo: usize, shmem_phys_hi: usize, flags: usize) -> SbiRet {
    unsafe { sbi_call_3(eid::NACL, FID_SET_SHMEM, shmem_phys_lo, shmem_phys_hi, flags) }
}

pub fn sync_csr(csr_num: usize) -> SbiRet {
    unsafe { sbi_call_1(eid::NACL, FID_SYNC_CSR, csr_num) }
}

pub fn sync_hfence(entry_index: usize) -> SbiRet {
    unsafe { sbi_call_1(eid::NACL, FID_SYNC_HFENCE, entry_index) }
}

pub fn sync_sret() -> SbiRet {
    unsafe { sbi_call_1(eid::NACL, FID_SYNC_SRET, 0) }
}
