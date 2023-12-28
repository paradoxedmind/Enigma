use lazy_static::lazy_static;
use x86_64::{
    structures::{
        gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
        tss::TaskStateSegment,
    },
    VirtAddr,
};

lazy_static! {
    // GDT contains segments of program. While segmentation is no longer supported in 64-bit mode,
    // the GDT still exists. It is mostly used for two things: Switching between kernel space and
    // user space, and loading a TSS structure.
    static ref GDT: ( GlobalDescriptorTable, Selector ) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment() );
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt, Selector{ code_selector, tss_selector })
    };
}

pub fn init() {
    use x86_64::instructions::segmentation::{Segment, CS};
    use x86_64::instructions::tables::load_tss;

    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.code_selector); // Reload Code Segment register
        load_tss(GDT.1.tss_selector);
    }
}

struct Selector {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0; // Defining Double Fault Stack Index

lazy_static! {
    //on x86_64 holds two stack tables( Interrupt Stack Table and Privilege Stack Table )
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();

        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end = stack_start + STACK_SIZE;
            stack_end // Writing Top address of stack because stacks on x86 grows downwards
        };
        tss
    };
}

