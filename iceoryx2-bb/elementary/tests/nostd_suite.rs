#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), no_main)]

use core::{
    alloc::{GlobalAlloc, Layout},
    ptr::NonNull,
};

use iceoryx2_bb_elementary_traits::allocator::BaseAllocator;
use iceoryx2_bb_memory::heap_allocator::HeapAllocator;

#[derive(Debug)]
pub struct GlobalHeapAllocator(HeapAllocator);

impl GlobalHeapAllocator {
    pub const fn new() -> Self {
        Self(HeapAllocator::new())
    }
}

unsafe impl GlobalAlloc for GlobalHeapAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        match self.0.allocate(layout) {
            Ok(ptr) => ptr.as_ptr() as *mut u8,
            Err(_) => core::ptr::null_mut(),
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if let Some(non_null) = NonNull::new(ptr) {
            self.0.deallocate(non_null, layout);
        }
    }
}

#[global_allocator]
static GLOBAL: GlobalHeapAllocator = GlobalHeapAllocator::new();

iceoryx2_pal_testing_nostd::test_suite! {
    "math_tests.rs" as math_tests,
}
