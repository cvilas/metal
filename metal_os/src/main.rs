#![no_std]  // Don't link the Rust standard library
#![no_main] // disable all Rust-level entry points`

use core::panic::PanicInfo;

/// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// The entry point, since the linker looks for a function named `_start` by default
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
