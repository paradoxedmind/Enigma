 # Arcane
## Operating System in Rust

- [x] [A Freestanding Rust Binary](https://os.phil-opp.com/freestanding-rust-binAary/) : Created Rust executable that does not link the standard library and Makes it possible to run it on bare metal without an underlying operating system.
- [x] [A Minimal Rust Kernel](https://os.phil-opp.com/minimal-rust-kernel/) : Created Minimal 64-bit Rust Kernel fot x86-architecture. Created a bootable disk image that prints "Hello World" to the screen.
- [x] [VGA Text Mode](https://os.phil-opp.com/vga-text-mode/) : Created Rust Module that encapsulates unsafety of writing to VGA text buffer thorugh memory mapping address `0xb8000` and presents a safe and convenient interface to the outside through `print!` and `println!` macros
- [x] [Testing](https://os.phil-opp.com/testing/) : Set up Rust's custom test framework for our rust kernel. Used to implement support for simple `#[test_case]` attribute. Using the `isa-debug-exit` device of QEMU, our test runner can exit QEMU after running the tests and report the test status. To print error messages to the console instead of the VGA buffer, we created a basic driver for the serial port. And explored intergration tests.
- [x] [CPU Exceptions](https://os.phil-opp.com/cpu-exceptions/) : Used `x86_64` crate to introduce Exception handling. Crate provides `x86-interrupt` calling convention and [InterruptDescriptorTable]() type which made handling process easier. Currently it just caught `breakpoint` exception (int3 instruction) and returns from it.
- [ ] [Double Faults](https://os.phil-opp.com/double-fault-exceptions/)
- [ ] [Hardware Interrupts](https://os.phil-opp.com/hardware-interrupts/)
- [ ] [Introduction to Paging](https://os.phil-opp.com/paging-introduction/)
- [ ] [Paging Implementation](https://os.phil-opp.com/paging-implementation/)
- [ ] [Heap Allocation](https://os.phil-opp.com/heap-allocation/)
- [ ] [Allocator Designs](https://os.phil-opp.com/allocator-designs/)
- [ ] [Async/Await](https://os.phil-opp.com/async-await/)
