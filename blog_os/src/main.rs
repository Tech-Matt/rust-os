#![no_std] // don't link the Rust standard Library
#![no_main] // disable all Rust levels entry points

use core::panic::PanicInfo;

// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
} 

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // This function is the entry point, since the linker looks for a function
	// named _start by default
	loop {}
	
}

