#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rosox::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

pub mod arch;
use arch::*;
use core::fmt::Write;
pub use core::panic::PanicInfo;
use vga_buffer::Color;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    color_print!(Color::Magenta, Color::Black, "rOSox");
    color_println!(
        Color::LightRed,
        Color::Black,
        " v{}\n",
        env!("CARGO_PKG_VERSION")
    );

    #[cfg(test)]
    test_main();

    loop {}
}

unsafe fn kernel_init() -> ! {
    panic!()
}
