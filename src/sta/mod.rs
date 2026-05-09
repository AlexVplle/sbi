use super::{SbiRet, eid, sbi_call_3};

const FID_SET_SHMEM: usize = 0;

pub fn set_shmem(shmem_phys_lo: usize, shmem_phys_hi: usize, flags: usize) -> SbiRet {
    unsafe { sbi_call_3(eid::STA, FID_SET_SHMEM, shmem_phys_lo, shmem_phys_hi, flags) }
}
