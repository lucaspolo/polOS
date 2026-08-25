use spin::{LazyLock, Mutex};
use uart_16550::backend::PioBackend;
use uart_16550::{Config, Uart16550Tty};

/// `Uart16550Tty` já inicializa o dispositivo, roda o self-test de loopback e
/// implementa `fmt::Write` traduzindo `\n` em `\r\n` — o que o antigo
/// `SerialPort` (uart_16550 0.2) fazia manualmente.
pub static SERIAL1: LazyLock<Mutex<Uart16550Tty<PioBackend>>> = LazyLock::new(|| {
    let serial_port = unsafe { Uart16550Tty::new_port(0x3F8, Config::default()) }
        .expect("failed to initialize serial port at 0x3F8");
    Mutex::new(serial_port)
});

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        SERIAL1
            .lock()
            .write_fmt(args)
            .expect("Printing to serial failed");
    });
}

#[macro_export]
macro_rules! serial_print {
	($($arg:tt)*) => {
		$crate::serial::_print(format_args!($($arg)*));
	};
}

#[macro_export]
macro_rules! serial_println {
	() => ($crate::serial_print!("\n"));
	($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
	($fmt:expr, $($arg:tt)*) => ($crate::serial_print! (
		concat!($fmt, "\n"), $($arg)*));
}
