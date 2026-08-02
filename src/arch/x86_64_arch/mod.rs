pub mod interrupts;
pub mod serial;
pub mod vga_buffer;

pub fn init() {
    interrupts::init_idt();
}
