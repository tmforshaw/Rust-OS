#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(operating_system::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(clippy::empty_loop)]

extern crate alloc;

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};

use bootloader::{bootinfo, entry_point, BootInfo};

use operating_system::{
    allocator, hlt_loop,
    memory::{self, translate_addr, BootInfoFrameAllocator},
    println,
    task::{executor::Executor, keyboard, simple_executor::SimpleExecutor, Task},
};
use x86_64::{
    structures::paging::{Page, Translate},
    VirtAddr,
};

mod panic;

entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World!");

    operating_system::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("Heap initialisation failed");

    #[cfg(test)]
    test_main();

    println!("No crash happened");

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task(42)));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
}

async fn async_number(i: u32) -> u32 {
    i
}

async fn example_task(i: u32) {
    let number = async_number(i).await;
    println!("async number: {}", number);
}
