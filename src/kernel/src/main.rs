#![no_std]
#![no_main]

use core::arch::asm;
use core::fmt::Write;
use core::panic::PanicInfo;
use limine::BaseRevision;
use limine::request::{RequestsEndMarker, RequestsStartMarker};
use uart_16550::SerialPort;

// Limine locates these requests via ELF sections, not symbol names,
// so #[unsafe(no_mangle)] is unnecessary on the static variables.
#[used]
#[unsafe(link_section = ".requests_start_marker")]
static START_MARKER: RequestsStartMarker = RequestsStartMarker::new();

#[used]
#[unsafe(link_section = ".requests")]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[unsafe(link_section = ".requests_end_marker")]
static END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

fn hlt_loop() -> ! {
    loop {
        unsafe {
            asm!("cli", "hlt", options(nomem, nostack, preserves_flags));
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // 1. Initialize serial output
    let mut serial_port = unsafe { SerialPort::new(0x3F8) };
    serial_port.init();

    let _ = writeln!(serial_port, "\r\n[KERNEL] Entered _start!\r");

    // 2. Verify Limine Base Revision
    if !BASE_REVISION.is_supported() {
        let _ = writeln!(
            serial_port,
            "[KERNEL] ERROR: Limine Base Revision not supported!\r"
        );
        hlt_loop();
    }

    // 3. Confirm protocol handoff succeeded
    let _ = writeln!(serial_port, "[KERNEL] Limine checks passed! Hello World!\r");

    hlt_loop();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    hlt_loop();
}

