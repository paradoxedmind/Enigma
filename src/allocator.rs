pub mod bump;
pub mod fixed_size_block;
pub mod linked_list;

use linked_list::LinkedListAllocator;
use spin::{Mutex, MutexGuard};
use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
    },
    VirtAddr,
};

use self::fixed_size_block::FixedSizeBlockAllocator;

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

/// A wraper around spin::Mutex to permit trait implementations.
pub struct Locked<A> {
    inner: Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Self {
            inner: Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> MutexGuard<A> {
        // So no data race can occur in multithreaded contexts
        self.inner.lock()
    }
}

/// Align the given address `addr` upwards to alignment `align`.
fn align_up(addr: usize, align: usize) -> usize {
    // let remainder = addr % alig;
    // if remainder == 0 {
    //     addr
    // } else {
    //     addr - remainder + alig
    // }
    (addr + align - 1) & !(align - 1)
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
//static ALLOCATOR: LockedHeap = LockedHeap::empty(); // -> linked_list_crate allocator
// static ALLOCATOR: Locked<BumpAllocator> = Locked::new(BumpAllocator::new()); // BumpAllocator
// static ALLOCATOR: Locked<LinkedListAllocator> = Locked::new(LinkedListAllocator::new());
static ALLOCATOR: Locked<FixedSizeBlockAllocator> = Locked::new(FixedSizeBlockAllocator::new());
