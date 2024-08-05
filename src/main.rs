#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(operating_system::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(clippy::empty_loop)]

mod panic;
mod serial;
mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!(
        "This is a testicle of the highest order aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\nWagwarn"
    );

    #[cfg(test)]
    test_main();

    loop {}
}
