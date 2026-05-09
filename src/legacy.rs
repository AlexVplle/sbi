use core::fmt::Write;
use super::{SbiRet, eid, hart_mask::HartMask, sbi_call_0, sbi_call_1, sbi_call_3, sbi_call_4};

pub struct SbiWriter;

impl Write for SbiWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            unsafe { sbi_call_1(eid::LEGACY_CONSOLE_PUTCHAR, 0, byte as usize) };
        }
        Ok(())
    }
}

pub fn shutdown() -> ! {
    unsafe { sbi_call_0(eid::LEGACY_SHUTDOWN, 0) };
    loop {}
}

pub fn remote_sfence_vma_asid(hart_mask: &HartMask, start: usize, size: usize, asid: usize) -> SbiRet {
    unsafe { sbi_call_4(eid::LEGACY_REMOTE_SFENCE_VMA_ASID, 0, hart_mask.as_ptr() as usize, start, size, asid) }
}

pub fn remote_sfence_vma(hart_mask: &HartMask, start: usize, size: usize) -> SbiRet {
    unsafe { sbi_call_3(eid::LEGACY_REMOTE_SFENCE_VMA, 0, hart_mask.as_ptr() as usize, start, size) }
}

pub fn remote_fence_i(hart_mask: &HartMask) -> SbiRet {
    unsafe { sbi_call_1(eid::LEGACY_REMOTE_FENCE_I, 0, hart_mask.as_ptr() as usize) }
}

pub fn send_ipi(hart_mask: &HartMask) -> SbiRet {
    unsafe { sbi_call_1(eid::LEGACY_SEND_IPI, 0, hart_mask.as_ptr() as usize) }
}

pub fn clear_ipi() -> SbiRet {
    unsafe { sbi_call_0(eid::LEGACY_CLEAR_IPI, 0) }
}

pub fn console_getchar() -> Option<u8> {
    let ret: SbiRet = unsafe { sbi_call_0(eid::LEGACY_CONSOLE_GETCHAR, 0) };
    if ret.value == usize::MAX { None } else { Some(ret.value as u8) }
}

pub fn set_timer(stime_value: u64) -> SbiRet {
    unsafe { sbi_call_1(eid::LEGACY_SET_TIMER, 0, stime_value as usize) }
}

pub fn console_print(s: &str) {
    let mut writer: SbiWriter = SbiWriter;
    writer.write_str(s).ok();
}
