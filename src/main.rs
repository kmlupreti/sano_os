#![no_std] // disable standard library
#![no_main] // don't use main function as entry point
use core::panic::PanicInfo;

static MESSAGE: &[u8] = b"Sano OS";

// entry point
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &byte) in MESSAGE.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}

// function called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
