#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(const_mut_refs)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(clippy::empty_loop)]

#[cfg(test)]
use bootloader::{entry_point, BootInfo};

use core::panic::PanicInfo;

pub mod allocator;
pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod serial;
pub mod task;
pub mod vga_buffer;

extern crate alloc;

pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{} => \t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

/// Entry point for `cargo test`
#[cfg(test)]
entry_point!(kernel_main);

#[cfg(test)]
fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    init();
    test_main();

    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };

    x86_64::instructions::interrupts::enable();
}

#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
