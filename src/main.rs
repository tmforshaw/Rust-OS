#![no_std]
#![no_main]

mod panic;
mod vga_buffer;

use vga_buffer::Writer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut writer = Writer::new();

    writer.write_string("This is a test");

    loop {}
}
