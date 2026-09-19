use crate::debugger::debug;
use lib::arch::hlt;

pub unsafe extern "C" fn main(_cpu: &limine::mp::Cpu) -> ! {
    debug("Hello from ASP");
    hlt()
}
