use core::fmt::Write;
use spin::{Mutex, Once};
use uart_16550::SerialPort;

static SERIAL: Once<Mutex<SerialPort>> = Once::new();

pub fn serial() -> &'static Mutex<SerialPort> {
    SERIAL.call_once(|| {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    })
}

pub fn debug(s: &str) {
    let _ = writeln!(serial().lock(), "{}", s);
}
