use super::{SbiRet, eid, sbi_call_0, sbi_call_1};

const FID_GET_SPEC_VERSION: usize = 0;
const FID_GET_IMPL_ID: usize = 1;
const FID_GET_IMPL_VERSION: usize = 2;
const FID_PROBE_EXTENSION: usize = 3;
const FID_GET_MVENDORID: usize = 4;
const FID_GET_MARCHID: usize = 5;
const FID_GET_MIMPID: usize = 6;

pub fn get_spec_version() -> SbiRet {
    unsafe { sbi_call_0(eid::BASE, FID_GET_SPEC_VERSION) }
}

pub fn get_impl_id() -> SbiRet {
    unsafe { sbi_call_0(eid::BASE, FID_GET_IMPL_ID) }
}

pub fn get_impl_version() -> SbiRet {
    unsafe { sbi_call_0(eid::BASE, FID_GET_IMPL_VERSION) }
}

pub fn probe_extension(eid: usize) -> bool {
    let ret: SbiRet = unsafe { sbi_call_1(eid::BASE, FID_PROBE_EXTENSION, eid) };
    ret.value != 0
}

pub fn get_mvendorid() -> SbiRet {
    unsafe { sbi_call_0(eid::BASE, FID_GET_MVENDORID) }
}

pub fn get_marchid() -> SbiRet {
    unsafe { sbi_call_0(eid::BASE, FID_GET_MARCHID) }
}

pub fn get_mimpid() -> SbiRet {
    unsafe { sbi_call_0(eid::BASE, FID_GET_MIMPID) }
}
