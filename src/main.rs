#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(operating_system::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(clippy::empty_loop)]

use operating_system::println;

mod panic;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World!");

    operating_system::init();

    fn so() {
        so()
    }

    so();

    #[cfg(test)]
    test_main();

    println!("No crash happened");

    loop {}
}
