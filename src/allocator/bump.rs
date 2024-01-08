// Bump Allocator:
// The most simple allocator design is a bump allocator (also known as stack allocator). It allocates
// memory linearly and only keeps track of the number of allocated bytes and the number of allocations.

// Biggest  Advantage:  it’s very fast. Compared to other allocator designs (see below) that need to
// actively look for a fitting memory block and perform various bookkeeping tasks on alloc and dealloc, a
//  bump allocator can be optimized to just a few assembly instructions. This makes bump allocators useful
//  for optimizing the allocation performance, for example when creating a virtual DOM library.

use core::ptr;

use super::{align_up, Locked};
use alloc::alloc::{GlobalAlloc, Layout};

pub struct BumpAllocator {
    heap_start: usize,  // ---|
    heap_end: usize,    // ---| To keep track of lower and upper bounds of heap memory regions
    next: usize,        // ---> Should always point to first unsused byte of the heap
    allocations: usize, // ---> counter for active allocations
}

impl BumpAllocator {
    /// Creates a new empty bump allocator.
    pub const fn new() -> Self {
        Self {
            heap_start: 0,
            heap_end: 0,
            next: 0,
            allocations: 0,
        }
    }

    /// Intializes the bump allocator with the given heap bounds
    ///
    /// This method is unsafe because the caller must ensure that the given
    /// memory range is unused. Also, this method must be called only once.
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.heap_start = heap_start;
        self.heap_end = heap_start + heap_size;
        self.next = heap_start;
    }
}

unsafe impl GlobalAlloc for Locked<BumpAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut bump = self.lock();

        let alloc_start = align_up(bump.next, layout.align());
        let alloc_end = match alloc_start.checked_add(layout.size()) {
            Some(end) => end,
            None => return ptr::null_mut(),
        };

        if alloc_end > bump.heap_end {
            ptr::null_mut() // Out of memory
        } else {
            bump.next = alloc_end;
            bump.allocations += 1;
            alloc_start as *mut u8
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        let mut bump = self.lock();

        bump.allocations -= 1;
        if bump.allocations == 0 {
            bump.next = bump.heap_start;
        }
    }
}
