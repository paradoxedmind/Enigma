#![no_std] // Disable Standard Library Linking
#![no_main] // Overwriting Entry Point i.e, `main`

use core::panic::PanicInfo;

/// This function is called on Panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Entry point which uses C calling Convention
// `_start` because default for most systems
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
