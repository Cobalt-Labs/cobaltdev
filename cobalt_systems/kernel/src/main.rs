#![no_std]
#![no_main]
#![forbid(unsafe_op_in_unsafe_fn)]


// use core::panic::PanicInfo;
// #[panic_handler]
// fn panic(_info: &PanicInfo) -> ! {
//     loop {
//         unsafe {
//             core::arch::asm!("hlt");
//         }
//     }
// }

/// Entry point
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    let message = b"Hello";

    for (i, &byte) in message.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0x07;
        }
    }

    loop {}
}