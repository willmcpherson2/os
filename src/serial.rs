use core::fmt;
use core::fmt::Write;
use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

#[macro_export]
macro_rules! serial {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! serialn {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial!(concat!($fmt, "\n"), $($arg)*));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    lazy_static! {
        static ref SERIAL1: Mutex<SerialPort> = {
            let mut serial_port = unsafe { SerialPort::new(0x3F8) };
            serial_port.init();
            Mutex::new(serial_port)
        };
    }

    SERIAL1
        .lock()
        .write_fmt(args)
        .expect("Printing to serial failed");
}
