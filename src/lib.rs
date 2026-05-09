#![no_std]

pub mod base;
pub mod cppc;
pub mod dbtr;
pub mod debug_console;
pub mod eid;
pub mod fwft;
pub mod hart_mask;
pub mod hsm;
pub mod ipi;
pub mod legacy;
pub mod mpxy;
pub mod nacl;
pub mod pmu;
pub mod rfence;
pub mod srst;
pub mod sse;
pub mod sta;
pub mod susp;
pub mod timer;

pub const EXTENSIONS: &[(&str, usize)] = &[
    ("LEGACY_SET_TIMER",              eid::LEGACY_SET_TIMER),
    ("LEGACY_CONSOLE_PUTCHAR",        eid::LEGACY_CONSOLE_PUTCHAR),
    ("LEGACY_CONSOLE_GETCHAR",        eid::LEGACY_CONSOLE_GETCHAR),
    ("LEGACY_CLEAR_IPI",              eid::LEGACY_CLEAR_IPI),
    ("LEGACY_SEND_IPI",               eid::LEGACY_SEND_IPI),
    ("LEGACY_REMOTE_FENCE_I",         eid::LEGACY_REMOTE_FENCE_I),
    ("LEGACY_REMOTE_SFENCE_VMA",      eid::LEGACY_REMOTE_SFENCE_VMA),
    ("LEGACY_REMOTE_SFENCE_VMA_ASID", eid::LEGACY_REMOTE_SFENCE_VMA_ASID),
    ("LEGACY_SHUTDOWN",               eid::LEGACY_SHUTDOWN),
    ("BASE",  eid::BASE),
    ("TIMER", eid::TIMER),
    ("IPI",   eid::IPI),
    ("RFENCE",eid::RFENCE),
    ("HSM",   eid::HSM),
    ("SRST",  eid::SRST),
    ("PMU",   eid::PMU),
    ("DBCN",  eid::DBCN),
    ("SUSP",  eid::SUSP),
    ("CPPC",  eid::CPPC),
    ("NACL",  eid::NACL),
    ("STA",   eid::STA),
    ("FWFT",  eid::FWFT),
    ("DBTR",  eid::DBTR),
    ("SSE",   eid::SSE),
    ("MPXY",  eid::MPXY),
];

#[repr(isize)]
pub enum SbiError {
    Success = 0,
    Failed = -1,
    NotSupported = -2,
    InvalidParam = -3,
    Denied = -4,
    InvalidAddress = -5,
    AlreadyAvailable = -6,
    AlreadyStarted = -7,
    AlreadyStopped = -8,
    NoSharedMemory = -9,
    InvalidState = -10,
    BadRange = -11,
    Timeout = -12,
    Io = -13,
    DeniedLocked = -14,
}

#[repr(C)]
pub struct SbiRet<T = usize> {
    pub error: SbiError,
    pub value: T,
}

pub(crate) unsafe fn sbi_call_0(eid: usize, fid: usize) -> SbiRet {
    let error: usize;
    let value: usize;
    unsafe {
        core::arch::asm!(
            "ecall",
            out("a0") error,
            out("a1") value,
            in("a6") fid,
            in("a7") eid,
        );
    }
    SbiRet { error: unsafe { core::mem::transmute(error as isize) }, value }
}

pub(crate) unsafe fn sbi_call_1(eid: usize, fid: usize, a0: usize) -> SbiRet {
    let error: usize;
    let value: usize;
    unsafe {
        core::arch::asm!(
            "ecall",
            inout("a0") a0 => error,
            out("a1") value,
            in("a6") fid,
            in("a7") eid,
        );
    }
    SbiRet { error: unsafe { core::mem::transmute(error as isize) }, value }
}

pub(crate) unsafe fn sbi_call_2(eid: usize, fid: usize, a0: usize, a1: usize) -> SbiRet {
    let error: usize;
    let value: usize;
    unsafe {
        core::arch::asm!(
            "ecall",
            inout("a0") a0 => error,
            inout("a1") a1 => value,
            in("a6") fid,
            in("a7") eid,
        );
    }
    SbiRet { error: unsafe { core::mem::transmute(error as isize) }, value }
}

pub(crate) unsafe fn sbi_call_3(eid: usize, fid: usize, a0: usize, a1: usize, a2: usize) -> SbiRet {
    let error: usize;
    let value: usize;
    unsafe {
        core::arch::asm!(
            "ecall",
            inout("a0") a0 => error,
            inout("a1") a1 => value,
            in("a2") a2,
            in("a6") fid,
            in("a7") eid,
        );
    }
    SbiRet { error: unsafe { core::mem::transmute(error as isize) }, value }
}

pub(crate) unsafe fn sbi_call_4(eid: usize, fid: usize, a0: usize, a1: usize, a2: usize, a3: usize) -> SbiRet {
    let error: usize;
    let value: usize;
    unsafe {
        core::arch::asm!(
            "ecall",
            inout("a0") a0 => error,
            inout("a1") a1 => value,
            in("a2") a2,
            in("a3") a3,
            in("a6") fid,
            in("a7") eid,
        );
    }
    SbiRet { error: unsafe { core::mem::transmute(error as isize) }, value }
}

pub(crate) unsafe fn sbi_call_5(eid: usize, fid: usize, a0: usize, a1: usize, a2: usize, a3: usize, a4: usize) -> SbiRet {
    let error: usize;
    let value: usize;
    unsafe {
        core::arch::asm!(
            "ecall",
            inout("a0") a0 => error,
            inout("a1") a1 => value,
            in("a2") a2,
            in("a3") a3,
            in("a4") a4,
            in("a6") fid,
            in("a7") eid,
        );
    }
    SbiRet { error: unsafe { core::mem::transmute(error as isize) }, value }
}

#[macro_export]
macro_rules! bitflags {
    ($name:ident: $type:ty { $($flag:ident = $bit:expr,)* }) => {
        #[derive(Copy, Clone, PartialEq, Eq)]
        pub struct $name($type);

        impl $name {
            pub const fn empty() -> Self { Self(0) }
            pub const fn bits(self) -> $type { self.0 }
            pub const fn contains(self, other: Self) -> bool { (self.0 & other.0) == other.0 }

            $(pub const $flag: Self = Self(1 << $bit);)*
        }

        impl core::ops::BitOr for $name {
            type Output = Self;
            fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
        }

        impl core::ops::BitAnd for $name {
            type Output = Self;
            fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
        }

        impl core::ops::BitOrAssign for $name {
            fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
        }

        impl core::ops::BitAndAssign for $name {
            fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
        }
    }
}
