use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;
use crate::println;


lazy_static! {
    // CPU reads IDT entry for Execption when Exception occurs
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        // Setiing handler function for break point exception which happens on int3
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load(); // Load IDT to CPU
}

// Ensures calling convention appropriate for exception handling
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("[EXCEPTION] BREAKPOINT\n{:#?}",stack_frame);
}



// Test Cases
#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}
