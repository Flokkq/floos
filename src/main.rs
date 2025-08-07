#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!();
    println!("Cheaters get banned! {}", 3);
    panic!();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);

    loop {}
}
