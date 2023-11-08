#![no_std] // Disable Standard Library Linking
#![no_main] // Disable Rust-level Entry Points

mod vga_buffer;

use core::panic::PanicInfo;

/// This function is called on Panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}


// Entry point which uses C calling Convention
// entry point because linker looks for function name `_start`
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}","!!");
    
    panic!("Some Panic Message");
    loop {}
}
