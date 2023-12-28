#![no_std] // Disable Standard Library Linking
#![no_main] // Disable Rust-level Entry Points
#![feature(custom_test_frameworks)]
#![test_runner(enigma::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod serial;
mod vga_buffer;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

// Entry Point Macro defines `_start` entry point for us and also checks that arguments passed are
// correct because our entry point will be called externally and our function signature will not be
// checked
entry_point!(kernel_main);

// BootInfo has info about `memory_map`(available and reserved memory regions) and
// `physical_memory_offset` (Adding this offset to physical address, we get Virtual Address)
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use enigma::memory;
    use enigma::memory::BootInfoFrameAllocator;
    use x86_64::VirtAddr;

    println!("Hello World{}", "!!");
    enigma::init(); // Load GDT and IDT

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let _mapper = unsafe { memory::init(phys_mem_offset) };
    let _frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    // FOR TESTING MAPPING OF PAGES TO FRAME
    //// Map an unused page
    //let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    //memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    //// Write The String `New!` to the screen through the new mapping
    //let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    //unsafe {
    //    page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e);
    //}

    #[cfg(test)]
    test_main();

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
