use crate::debugger::debug;
use lib::arch::hlt;

pub fn main() -> ! {
    debug("Hello From the BSP");
    hlt()
}
