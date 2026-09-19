use core::arch::asm;

pub fn hlt() -> ! {
    loop {
        unsafe {
            asm!("cli", "hlt", options(nomem, nostack, preserves_flags));
        }
    }
}
