#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

mod panic;
mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!(
        "This is a testicle of the highest order aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\nWagwarn"
    );

    loop {}
}
