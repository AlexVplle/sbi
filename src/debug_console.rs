use super::{SbiRet, eid, sbi_call_1, sbi_call_3};

const FID_WRITE: usize = 0;
const FID_READ: usize = 1;
const FID_WRITE_BYTE: usize = 2;

pub fn write(bytes: &[u8]) -> SbiRet {
    unsafe {
        sbi_call_3(
            eid::DBCN,
            FID_WRITE,
            bytes.len(),
            bytes.as_ptr() as usize,
            0,
        )
    }
}

pub fn read(buf: &mut [u8]) -> SbiRet {
    unsafe {
        sbi_call_3(
            eid::DBCN,
            FID_READ,
            buf.len(),
            buf.as_mut_ptr() as usize,
            0,
        )
    }
}

pub fn write_byte(byte: u8) -> SbiRet {
    unsafe { sbi_call_1(eid::DBCN, FID_WRITE_BYTE, byte as usize) }
}

pub fn print(s: &str) -> SbiRet {
    write(s.as_bytes())
}
