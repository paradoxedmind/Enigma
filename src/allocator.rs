use core::{
    alloc::{GlobalAlloc, Layout},
    ptr::null_mut,
};
use linked_list_allocator::LockedHeap;
use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
    },
    VirtAddr,
};

pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe {
            mapper.map_to(page, frame, flags, frame_allocator)?.flush();
        }
    }

    unsafe {
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }

    Ok(())
}

// To tells rust compiler which allocator instance it should use as global heap
// allocator.the attribute is only applicable to a static that implements the
// GlobalAlloc trait
#[global_allocator]
// `LockedHeap` uses the spinning_top::Spinlock type for synchronization. This is required because multiple threads
// could access the ALLOCATOR static at the same time. As always, when using a spinlock or a mutex, we need
// to be careful to not accidentally cause a deadlock. This means that we shouldn’t perform any allocations
// in interrupt handlers, since they can run at an arbitrary time and might interrupt an in-progress
// allocation.
static ALLOCATOR: LockedHeap = LockedHeap::empty();
