use crate::{gdt, print, println, hlt_loop};
use lazy_static::lazy_static;
use pc_keyboard::{Keyboard, layouts, ScancodeSet1, HandleControl, DecodedKey};
use pic8259::ChainedPics; // Abstraction for Primary/Secondary PICs
use spin::Mutex;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

// offsets for PICs in range 32-47 because default are already occupied by CPU Exceptions
pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;
// NOTE : Interrupt controller work asynchronously so now we concorruency in our kernel
pub static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

lazy_static! {
    // CPU reads IDT entry for Execption when Exception occurs
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        // Setiing handler function for exceptions
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX); // Switch stack on double fault
        }
        // Register Timer Interrupt handler, enabled by default if  external interrupts are enabled
        idt[InterruptIndex::Timer.as_usize()]
            .set_handler_fn(timer_interrupt_handler);

        // Register Keyboard Interrupt Handler
        idt[InterruptIndex::Keyboard.as_usize()]
            .set_handler_fn(keyboard_interrupt_handler);
        
        // Setting Page Fault Handler
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load(); // Load IDT to CPU
}

// `x86-interrupt` FFI Ensures calling convention appropriate for exception handling
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("[EXCEPTION] BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn page_fault_handler(stack_frame: InterruptStackFrame, error_code: PageFaultErrorCode) {
    use x86_64::registers::control::Cr2;

    println!("[EXCEPTION] PAGE FAULT");
    println!("Accessed Address: {:?}",Cr2::read()); // Cr2 Register containes address that caused Fault        
    println!("Error Code: {:?}", error_code);
    println!("{:#?}",stack_frame);

    hlt_loop(); // Continue only after resolving page Fault
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("[EXCEPTION] DOUBLE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    print!(".");

    unsafe {
        // PICs require `end of interrupt` signal from handler so that it can know interrupt was
        // handled and system is ready to receive next interrupt
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use x86_64::instructions::port::Port;
    
    // NOTE : If we don't read key, next key press will not happen

    // Create Keyboard object with US keyboard layout and the scancode set 1
    // PS/2 keyboard emulate scancode set 1 (IBM XT)
    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = 
            Mutex::new(Keyboard::new(ScancodeSet1::new(), layouts::Us104Key, HandleControl::Ignore));
    }

    let mut keyboard = KEYBOARD.lock();
    // To know which key was pressed, we read from data port of PS/2 controller(keyboard interrupt
    // controller), which is I/O port `0x60`
    let mut port = Port::new(0x60);

    // Scan code is data that most computer keyboards send to computer about keys been pressed
    let scan_code: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scan_code) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => print!("{}",character),
                DecodedKey::RawKey(key) => print!("{:?}",key),
            }
        }
    }

    unsafe {
        // Set EOI signal so next interrupt can be received
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET, // (0 + 32) timer uses line 0 of primary PIC
    Keyboard,             // (33) keyboard uses line 1 of primary PIC
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        // because we use it as index on idt
        usize::from(self.as_u8())
    }
}

// Test Cases
#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}
