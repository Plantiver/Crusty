#![no_std]
#![no_main]

use core::panic::PanicInfo;
use limine::BaseRevision;

// Ensure Limine understands our kernel protocol
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // Check if the bootloader matches our expected revision
    assert!(BASE_REVISION.is_supported());

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
