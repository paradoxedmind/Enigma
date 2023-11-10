#![no_std] // Disable Standard Library Linking
#![no_main] // Disable Rust-level Entry Points
#![feature(custom_test_frameworks)]
#![test_runner(arcane::test_runner)]
#![reexport_test_harness_main = "test_main"]


mod vga_buffer;
mod serial;

use core::panic::PanicInfo;

// Entry point which uses C calling Convention
// entry point because linker looks for function name `_start`
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}","!!");

    arcane::init(); // Load IDT

    x86_64::instructions::interrupts::int3(); // Invoke breakpoint exception

    #[cfg(test)]
    test_main();
    
    loop {}
}


/// This function is called on Panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)] // This panic handler for test mode
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    arcane::test_panic_handler(info)
}
