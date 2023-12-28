 # Enigma
## Operating System in Rust

- [x] [A Freestanding Rust Binary](https://os.phil-opp.com/freestanding-rust-binAary/) : Created Rust executable that does not link the standard library and Makes it possible to run it on bare metal without an underlying operating system.
- [x] [A Minimal Rust Kernel](https://os.phil-opp.com/minimal-rust-kernel/) : Created Minimal 64-bit Rust Kernel for x86-architecture. Created a bootable disk image that prints "Hello World" to the screen.
- [x] [VGA Text Mode](https://os.phil-opp.com/vga-text-mode/) : Created Rust Module that encapsulates unsafety of writing to VGA text buffer through memory mapping address `0xb8000` and presents a safe and convenient interface to the outside through `print!` and `println!` macros.
- [x] [Testing](https://os.phil-opp.com/testing/) : Set up Rust's custom test framework for our rust kernel. Used to implement support for simple `#[test_case]` attribute. Using the `isa-debug-exit` device of QEMU, our test runner can exit QEMU after running the tests and report the test status. To print error messages to the console instead of the VGA buffer, we created a basic driver for the serial port. And explored intergration tests.
- [x] [CPU Exceptions](https://os.phil-opp.com/cpu-exceptions/) : Used `x86_64` crate to introduce Exception handling. Crate provides `x86-interrupt` calling convention and [InterruptDescriptorTable](https://docs.rs/x86_64/0.14.2/x86_64/structures/idt/struct.InterruptDescriptorTable.html) type which made handling process easier. Currently it just caught `breakpoint` exception (int3 instruction) and returns from it.
- [x] [Double Faults](https://os.phil-opp.com/double-fault-exceptions/) : Learned what a Double Fault is and under which conditions it occurs. Added Basic double fault handler that prints an error message and added integration test for it. Also enabled hardware-supported stack switching on double fault exceptions so that it also works on stack overflow. Also learned about TSS(Task State Segment), IST(Interrupt Stack Table) and GDT(Global Descriptor Table).
- [x] [Hardware Interrupts](https://os.phil-opp.com/hardware-interrupts/) : Enabled and handled External Interrupts. learned 8259 PIC and its primary/secondary layout, the remapping of the interrupt numbers, and the "end of interrupt" signal. Implemented handlers for the hardware timer and keyboard and added `hlt` instruction, which halts the CPU until the next interrupt.
- [x] [Introduction to Paging](https://os.phil-opp.com/paging-introduction/) : Learned about Memory Protection techniques: segmentation and paging. Paging stores mapping info for pages in page tables with one or more levels. The x86-64 uses 4-level page tables and a page size of 4KiB. The hardware automatically walks the page tables and caches the resulting translations in the translation lookaside buffer (TLB). This buffer is not updated transparently and needs to be flushed manually on page table changes. Learned that Our kernel already runs on top of paging by bootloader and illegal memory access cause page fault exceptions. Now We can't access active page table because `Cr3` register stores physical address of Table that we can't access directly from our kernel.
- [ ] [Paging Implementation](https://os.phil-opp.com/paging-implementation/) : Learned about different techniques to access physical frames of page tables, including identity mapping, mapping of the complete physical memory, temporary mapping, and recursive page tables. (Somethings are still not quite clear to me). We chose to map the complete physical memory since it's simple, portable and powerful. To access physical memory memory using page table we used `bootloader` crate which supports creating the required mapping through optional cargo crate features. It passes the required information to our kernel in the form of a &BootInfo argument to our entry point function. We first manually traversed the page tables to implement a translation function, and the used the `MappedPageTable` type of `x86_64` crate. also learned how to create new mappings in the page table and how to create the necessary `FrameAllocator` on top of the memory map passed by the bootloader.
- [ ] [Heap Allocation](https://os.phil-opp.com/heap-allocation/)
- [ ] [Allocator Designs](https://os.phil-opp.com/allocator-designs/)
- [ ] [Async/Await](https://os.phil-opp.com/async-await/)
