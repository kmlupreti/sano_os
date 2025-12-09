#![no_std] // disable standard library
#![no_main] // don't use main function as entry point
use core::panic::PanicInfo;

// function called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// entry point
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {}
}
