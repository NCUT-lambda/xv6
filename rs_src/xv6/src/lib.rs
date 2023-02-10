#![no_std]
#![allow(clippy::missing_safety_doc)]

use core::ffi;

extern "C" {
    pub fn c_main() -> ffi::c_void;
}

#[no_mangle]

pub unsafe extern "C" fn main() -> ffi::c_void {
    c_main()
}
