#![no_std]
#![no_main]

pub mod apscore;
pub mod bspcore;
pub mod debugger;
pub mod once;
pub mod requests;
pub mod utils;

use debugger::debug;
use requests::*;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    if !BASE_REVISION.is_supported() {
        panic!("[KERNEL] Base Revision not supported, Halting\n")
    };

    debug("[KERNEL] Hey");
    once::start_cores()
}
