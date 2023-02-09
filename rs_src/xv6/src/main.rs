#![no_std]
#![cfg_attr(not(test), no_main)]

use core::panic::PanicInfo;

#[allow(unused_imports)]
use xv6;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
