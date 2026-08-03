use crate::*;

/// This function is called on panic.
#[cfg(target_arch = "x86_64")]
#[cfg(not(test))]
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(target_arch = "x86_64")]
#[cfg(test)]
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

#[cfg(target_arch = "aarch64")]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    cpu::wait_forever()
}
