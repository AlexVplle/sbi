pub mod reg;

use reg::CppcReg;
use super::{SbiRet, eid, sbi_call_1, sbi_call_2};

const FID_PROBE: usize = 0;
const FID_READ: usize = 1;
const FID_READ_HI: usize = 2;
const FID_WRITE: usize = 3;

pub fn probe(reg: CppcReg) -> SbiRet {
    unsafe { sbi_call_1(eid::CPPC, FID_PROBE, reg as usize) }
}

pub fn read(reg: CppcReg) -> SbiRet {
    unsafe { sbi_call_1(eid::CPPC, FID_READ, reg as usize) }
}

pub fn read_hi(reg: CppcReg) -> SbiRet {
    unsafe { sbi_call_1(eid::CPPC, FID_READ_HI, reg as usize) }
}

pub fn write(reg: CppcReg, val: u64) -> SbiRet {
    unsafe { sbi_call_2(eid::CPPC, FID_WRITE, reg as usize, val as usize) }
}
