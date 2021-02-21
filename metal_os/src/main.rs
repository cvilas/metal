#![no_std]  // Don't link the Rust standard library
#![no_main] // disable all Rust-level entry points`

mod vga_buffer;
use core::panic::PanicInfo;

/// This function is called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

// The entry point, since the linker looks for a function named `_start` by default
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    panic!("Some panic message");
    loop {}
}
