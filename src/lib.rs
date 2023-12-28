#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)] // Default Test Framework will not work
#![test_runner(crate::test_runner)] // Test Runner which runs our tests
#![reexport_test_harness_main = "test_main"] // Test require main_function
#![feature(abi_x86_interrupt)]

pub mod gdt;
pub mod interrupts;
pub mod serial;
pub mod vga_buffer;
pub mod memory;

use core::panic::PanicInfo;

pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe {
        // Intialize PICs could cause undefined behaviour if PIC is misconfigured
        interrupts::PICS.lock().initialize();
    };
    x86_64::instructions::interrupts::enable(); // Enable External Interrupts
}

pub fn hlt_loop() -> ! {
    loop {
        // Halt the CPU until next interrupt arrives, allows CPU to enter sleep state consuming
        // less energy
        x86_64::instructions::hlt();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    // Exit with exit code different from QEMU default code to differ test
    Success = 0x10, // 33 after exit
    Failed = 0x11,  // 35 after exit
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        // 0xf4 iobase value which we passed as argument to QEMU
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T: Fn()> Testable for T {
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>()); // Function name
        self();
        serial_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

#[cfg(test)]
use bootloader::{entry_point, BootInfo};

#[cfg(test)]
entry_point!(test_kernel_main);

/// Entry Point for `cargo test`
#[cfg(test)]
#[no_mangle]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    init();
    test_main();
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info);
}
