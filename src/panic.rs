use core::panic::PanicInfo;

use crate::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}

#[cfg(test)]
#[panic_handler]

fn panic(info: &PanicInfo) -> ! {
    use crate::{exit_qemu, serial_println};

    serial_println!("[failed]");
    serial_println!("Error: {}\n", info);

    exit_qemu(crate::QemuExitCode::Failed);
    loop {}
}
