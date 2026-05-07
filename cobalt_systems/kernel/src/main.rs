#![no_std]
#![no_main]
#![forbid(unsafe_op_in_unsafe_fn)]

use core::panic::PanicInfo;
use bootloader_api::{entry_point, BootInfo};

mod vga_buffer;

entry_point!(kernel_main);

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    vga_buffer::print_something();

    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}