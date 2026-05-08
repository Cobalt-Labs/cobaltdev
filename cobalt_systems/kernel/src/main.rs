#![no_std]
#![no_main]
#![forbid(unsafe_op_in_unsafe_fn)]

use core::panic::PanicInfo;
use bootloader_api::{entry_point, BootInfo};

mod vga_buffer;
mod gdt;
mod interrupts;
mod keyboard;
mod shell;

entry_point!(kernel_main);

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    println!("Initializing Cobalt Kernel...");
    
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    println!("Kernel systems initialized.");
    shell::init_shell();

    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}