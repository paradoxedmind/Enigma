#![no_std] // Disable Standard Library Linking
#![no_main] // Disable Rust-level Entry Points

use core::panic::PanicInfo;

/// This function is called on Panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello, World!";

// Entry point which uses C calling Convention
// entry point because linker looks for function name `_start`
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8; // Raw Pointer at VGA Start Address

    for (i, &byte) in HELLO.iter().enumerate() {

        unsafe {
            *vga_buffer.offset(i as isize * 2 ) = byte; // Character
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // Color
        }
    }

    loop {}
}
