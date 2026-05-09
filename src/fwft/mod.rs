pub mod feature;

use feature::FwftFeature;
use super::{SbiRet, eid, sbi_call_1, sbi_call_3};

const FID_SET: usize = 0;
const FID_GET: usize = 1;

pub fn set(feature: FwftFeature, value: usize, flags: usize) -> SbiRet {
    unsafe { sbi_call_3(eid::FWFT, FID_SET, feature as usize, value, flags) }
}

pub fn get(feature: FwftFeature) -> SbiRet {
    unsafe { sbi_call_1(eid::FWFT, FID_GET, feature as usize) }
}
