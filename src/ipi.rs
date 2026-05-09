use super::{SbiRet, eid, hart_mask::HartMask, sbi_call_2};

const FID_SEND_IPI: usize = 0;

pub fn send_ipi(hart_mask: &HartMask) -> SbiRet {
    unsafe { sbi_call_2(eid::IPI, FID_SEND_IPI, hart_mask.mask, hart_mask.base) }
}
