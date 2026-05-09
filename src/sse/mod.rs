pub mod attr;

use super::{SbiRet, eid, sbi_call_1, sbi_call_2, sbi_call_3, sbi_call_4};

const FID_READ: usize = 0;
const FID_READ_ATTRS: usize = 1;
const FID_WRITE_ATTRS: usize = 2;
const FID_REGISTER: usize = 3;
const FID_UNREGISTER: usize = 4;
const FID_ENABLE: usize = 5;
const FID_DISABLE: usize = 6;
const FID_COMPLETE: usize = 7;
const FID_INJECT: usize = 8;
const FID_UNMASK: usize = 9;
const FID_MASK: usize = 10;

pub fn read(event_id: u32) -> SbiRet {
    unsafe { sbi_call_1(eid::SSE, FID_READ, event_id as usize) }
}

pub fn read_attrs(event_id: u32, attr_count: usize, output_phys_lo: usize, output_phys_hi: usize) -> SbiRet {
    unsafe { sbi_call_4(eid::SSE, FID_READ_ATTRS, event_id as usize, attr_count, output_phys_lo, output_phys_hi) }
}

pub fn write_attrs(event_id: u32, attr_count: usize, input_phys_lo: usize, input_phys_hi: usize) -> SbiRet {
    unsafe { sbi_call_4(eid::SSE, FID_WRITE_ATTRS, event_id as usize, attr_count, input_phys_lo, input_phys_hi) }
}

pub fn register(event_id: u32, handler_entry_pc: usize, handler_entry_arg: usize) -> SbiRet {
    unsafe { sbi_call_3(eid::SSE, FID_REGISTER, event_id as usize, handler_entry_pc, handler_entry_arg) }
}

pub fn unregister(event_id: u32) -> SbiRet {
    unsafe { sbi_call_1(eid::SSE, FID_UNREGISTER, event_id as usize) }
}

pub fn enable(event_id: u32) -> SbiRet {
    unsafe { sbi_call_1(eid::SSE, FID_ENABLE, event_id as usize) }
}

pub fn disable(event_id: u32) -> SbiRet {
    unsafe { sbi_call_1(eid::SSE, FID_DISABLE, event_id as usize) }
}

pub fn complete(event_id: u32, code: usize) -> SbiRet {
    unsafe { sbi_call_2(eid::SSE, FID_COMPLETE, event_id as usize, code) }
}

pub fn inject(event_id: u32, hart_id: usize) -> SbiRet {
    unsafe { sbi_call_2(eid::SSE, FID_INJECT, event_id as usize, hart_id) }
}

pub fn unmask(hart_id: usize) -> SbiRet {
    unsafe { sbi_call_1(eid::SSE, FID_UNMASK, hart_id) }
}

pub fn mask(hart_id: usize) -> SbiRet {
    unsafe { sbi_call_1(eid::SSE, FID_MASK, hart_id) }
}
