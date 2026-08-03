//! Processor code.

#[cfg(target_arch = "aarch64")]
#[path = "arch/aarch64_arch/cpu.rs"]
mod arch_cpu;

mod boot;

//--------------------------------------------------------------------------------------------------
// Architectural Public Reexports
//--------------------------------------------------------------------------------------------------
#[cfg(target_arch = "aarch64")]
pub use arch_cpu::wait_forever;
