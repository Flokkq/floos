#![no_std]
#![no_main]

mod vga_buffer;

use core::{fmt::Write, panic::PanicInfo};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    vga_buffer::WRITER.lock().write_str("Hello again");

    write!(
        vga_buffer::WRITER.lock(),
        ", some numbers: {} {}",
        42,
        1.337
    )
    .unwrap();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
