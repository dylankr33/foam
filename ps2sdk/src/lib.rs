#![no_std]

use core::{
    alloc::GlobalAlloc, ffi::c_void, panic::PanicInfo, prelude::rust_2024::global_allocator,
};
extern crate alloc;
pub mod sys {
    pub mod ee {
        pub use ps2sdk_ee_sys::*;
    }
}

#[global_allocator]
static ALLOCATOR: Ps2Alloc = Ps2Alloc;

struct Ps2Alloc;

unsafe impl GlobalAlloc for Ps2Alloc {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        unsafe { sys::ee::malloc(layout.size() as u32) as *mut u8 }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        unsafe {
            sys::ee::free(ptr as *mut c_void);
        }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe {
        sys::ee::printf("Panic!!!".as_ptr() as *const i8);
    }
    loop {}
}
