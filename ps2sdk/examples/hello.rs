#![no_std]
#![no_main]
use ps2sdk::sys::ee;

#[unsafe(no_mangle)]
extern "C" fn _start() {
    unsafe {
        ee::printf("hello\n".as_bytes().as_ptr() as *const i8);
    }
}
