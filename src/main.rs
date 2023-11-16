#![no_std] // Disable Standard Library Linking
#![no_main] // Disable Rust-level Entry Points
#![feature(custom_test_frameworks)]
#![test_runner(enigma::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod serial;
mod vga_buffer;

use core::panic::PanicInfo;

// Entry point which uses C calling Convention
// entry point because linker looks for function name `_start`
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!!");

    enigma::init(); // Load GDT and IDT

    #[cfg(test)]
    test_main();

    enigma::hlt_loop(); // Instead of endless loop, hlt till next interrupt
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
    enigma::test_panic_handler(info)
}
