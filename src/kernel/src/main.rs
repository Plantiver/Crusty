#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;
use limine::{BaseRevision, RequestsEndMarker, RequestsStartMarker};

// 1. Limine 0.6 REQUIRES these start and end markers
#[used]
#[unsafe(link_section = ".requests_start_marker")]
static START_MARKER: RequestsStartMarker = RequestsStartMarker::new();

#[used]
#[unsafe(link_section = ".requests_end_marker")]
static END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

// 2. Declare the request normally in the middle
#[used]
#[unsafe(link_section = ".requests")]
static BASE_REVISION: BaseRevision = BaseRevision::new();

fn hlt_loop() -> ! {
    loop {
        // "cli" disables interrupts so a random timer doesn't wake the CPU and crash it
        unsafe {
            asm!("cli", "hlt", options(nomem, nostack, preserves_flags));
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // 3. DO NOT use assert! here yet (it might trigger SSE formatting and triple fault)
    if !BASE_REVISION.is_supported() {
        hlt_loop(); // If unsupported, safely halt instead of panicking
    }

    // We passed the check! Halt safely.
    hlt_loop();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    hlt_loop();
}
