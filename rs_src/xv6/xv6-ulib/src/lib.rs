#![no_std]
#![allow(clippy::missing_safety_doc)]

use core::alloc::{GlobalAlloc, Layout};
use core::{ffi, mem};

extern crate xv6_user_panic;

use buddy_system_allocator::LockedHeap;

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap<32> = LockedHeap::<32>::empty();

#[no_mangle]
pub unsafe fn init_global_allocator() {
    const KERNEL_HEAP_SIZE: usize = 0x0080_0000;
    const MACHINE_ALIGN: usize = mem::size_of::<usize>();
    const HEAP_BLOCK: usize = KERNEL_HEAP_SIZE / MACHINE_ALIGN;
    static mut HEAP: [usize; HEAP_BLOCK] = [0; HEAP_BLOCK];
    HEAP_ALLOCATOR
        .lock()
        .init(HEAP.as_ptr() as usize, HEAP_BLOCK * MACHINE_ALIGN);
}

const C_ALIGN: usize = 8;

#[no_mangle]
pub unsafe extern "C" fn malloc(size: ffi::c_uint) -> *mut ffi::c_void {
    HEAP_ALLOCATOR.alloc(Layout::from_size_align_unchecked(size as _, C_ALIGN)) as _
}

#[no_mangle]
pub unsafe extern "C" fn free(_ptr: *mut ffi::c_void) {}
