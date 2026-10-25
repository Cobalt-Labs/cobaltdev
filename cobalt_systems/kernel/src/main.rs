#![no_std]
#![no_main]
#![forbid(unsafe_op_in_unsafe_fn)]
#![feature(abi_x86_interrupt)]

use bootloader_api::{entry_point, BootInfo};
use core::panic::PanicInfo;

mod gdt;
mod interrupts;
mod keyboard;
mod shell;
mod vga_buffer;

entry_point!(kernel_main);

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {
        // SAFETY: Halting the CPU until the next interrupt is safe.
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    println!("Initializing Cobalt Kernel...");

    gdt::init();
    interrupts::init_idt();
    // SAFETY: Initializing the PICs is safe as long as the offsets don't conflict with exceptions.
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    println!("Kernel systems initialized.");
    shell::init_shell();

    loop {
        // SAFETY: Halting the CPU until the next interrupt is safe.
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}
