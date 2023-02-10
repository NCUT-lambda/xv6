#![no_std]
#![cfg_attr(not(test), no_main)]

use core::panic::PanicInfo;

#[allow(unused_imports, clippy::single_component_path_imports)]
use xv6_kernel;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
