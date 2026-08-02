#[cfg(target_arch = "x86_64")]
pub mod x86_64_arch;

#[cfg(target_arch = "x86_64")]
pub use x86_64_arch::*;

#[cfg(target_arch = "aarch64")]
pub mod aarch64;

#[cfg(target_arch = "aarch64")]
pub use aarch64::*;

pub fn init() {
    #[cfg(target_arch = "x86_64")]
    x86_64_arch::init();

    #[cfg(target_arch = "aarch64")]
    aarch64::init();
}
