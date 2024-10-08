use core::panic::PanicInfo;

#[cfg(not(test))]
use crate::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    operating_system::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    operating_system::test_panic_handler(info)
}
