#![no_std] // Disable Standard Library Linking
#![no_main] // Disable Rust-level Entry Points
#![feature(custom_test_frameworks)]
#![test_runner(enigma::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

mod serial;
mod vga_buffer;

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

// Entry Point Macro defines `_start` entry point for us and also checks that arguments passed are
// correct because our entry point will be called externally and our function signature will not be
// checked
entry_point!(kernel_main);

// BootInfo has info about `memory_map`(available and reserved memory regions) and
// `physical_memory_offset` (Adding this offset to physical address, we get Virtual Address)
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use enigma::allocator;
    use enigma::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    println!("Hello World{}", "!!");
    enigma::init(); // Load GDT and IDT

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    // Create a dynamically sized vector
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    // Create a reference counted vector -> will be freed when count reaches 0
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!(
        "Current reference count is {}",
        Rc::strong_count(&cloned_reference)
    );
    core::mem::drop(reference_counted);
    println!(
        "reference count is {} now",
        Rc::strong_count(&cloned_reference)
    );

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    enigma::hlt_loop(); // Instead of endless loop, halt till next interrupt
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
