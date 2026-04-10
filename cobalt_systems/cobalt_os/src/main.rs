#![no_std]
#![no_main]

use core::panic::PanicInfo; //rm if unused
use core::ptr::offset;
use core::result::Result; // Importing Result explicitly
use core::alloc;
/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// The entry point of the kernel.
/// `#[no_mangle]` ensures the function name is not mangled, which is essential for linking.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    // Write "Hello" to the VGA buffer
    let message = b"Hello";
    for (i, &byte) in message.iter().enumerate() {
        unsafe {
            // Each character is followed by a color byte (0x07 = light grey on black background)
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0x07;
        }
    }

    loop {}
}
