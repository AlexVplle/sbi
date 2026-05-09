# sbi

RISC-V SBI (Supervisor Binary Interface) bindings.

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
