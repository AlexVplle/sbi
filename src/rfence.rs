use super::{SbiRet, eid, hart_mask::HartMask, sbi_call_2, sbi_call_4, sbi_call_5};

const FID_REMOTE_FENCE_I: usize = 0;
const FID_REMOTE_SFENCE_VMA: usize = 1;
const FID_REMOTE_SFENCE_VMA_ASID: usize = 2;
const FID_REMOTE_HFENCE_GVMA_VMID: usize = 3;
const FID_REMOTE_HFENCE_GVMA: usize = 4;
const FID_REMOTE_HFENCE_VVMA_ASID: usize = 5;
const FID_REMOTE_HFENCE_VVMA: usize = 6;

pub fn remote_fence_i(hart_mask: &HartMask) -> SbiRet {
    unsafe { sbi_call_2(eid::RFENCE, FID_REMOTE_FENCE_I, hart_mask.mask, hart_mask.base) }
}

pub fn remote_sfence_vma(hart_mask: &HartMask, start: usize, size: usize) -> SbiRet {
    unsafe { sbi_call_4(eid::RFENCE, FID_REMOTE_SFENCE_VMA, hart_mask.mask, hart_mask.base, start, size) }
}

pub fn remote_sfence_vma_asid(hart_mask: &HartMask, start: usize, size: usize, asid: usize) -> SbiRet {
    unsafe { sbi_call_5(eid::RFENCE, FID_REMOTE_SFENCE_VMA_ASID, hart_mask.mask, hart_mask.base, start, size, asid) }
}

pub fn remote_hfence_vvma(hart_mask: &HartMask, start: usize, size: usize) -> SbiRet {
    unsafe { sbi_call_4(eid::RFENCE, FID_REMOTE_HFENCE_VVMA, hart_mask.mask, hart_mask.base, start, size) }
}

pub fn remote_hfence_vvma_asid(hart_mask: &HartMask, start: usize, size: usize, asid: usize) -> SbiRet {
    unsafe { sbi_call_5(eid::RFENCE, FID_REMOTE_HFENCE_VVMA_ASID, hart_mask.mask, hart_mask.base, start, size, asid) }
}

pub fn remote_hfence_gvma(hart_mask: &HartMask, start: usize, size: usize) -> SbiRet {
    unsafe { sbi_call_4(eid::RFENCE, FID_REMOTE_HFENCE_GVMA, hart_mask.mask, hart_mask.base, start, size) }
}

pub fn remote_hfence_gvma_vmid(hart_mask: &HartMask, start: usize, size: usize, vmid: usize) -> SbiRet {
    unsafe { sbi_call_5(eid::RFENCE, FID_REMOTE_HFENCE_GVMA_VMID, hart_mask.mask, hart_mask.base, start, size, vmid) }
}
