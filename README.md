# sbi

RISC-V SBI (Supervisor Binary Interface) bindings.

Implements [SBI specification v3.0](https://github.com/riscv-non-isa/riscv-sbi-doc/releases/download/v3.0/riscv-sbi.pdf).

## Extensions

| Extension | Module |
|-----------|--------|
| Base | `sbi::base` |
| Legacy | `sbi::legacy` |
| Timer (TIME) | `sbi::timer` |
| IPI (sPI) | `sbi::ipi` |
| Remote Fence (RFENCE) | `sbi::rfence` |
| Hart State Management (HSM) | `sbi::hsm` |
| System Reset (SRST) | `sbi::srst` |
| Debug Console (DBCN) | `sbi::debug_console` |
| Performance Monitoring (PMU) | `sbi::pmu` |

## Usage

```toml
[dependencies]
sbi = { git = "https://github.com/AlexVplle/sbi.git" }
```
