use crate::debugger::debug;
use core::panic::PanicInfo;
use lib::arch::hlt;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    debug("[KERNEL] Panicked:\n");
    if let Some(s) = info.message().as_str() {
        debug(s)
    }
    hlt();
}
