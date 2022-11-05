use core::fmt::{self, Write};
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
const VGA_ADDRESS: i32 = 0xb8000;

type Buffer = [[Volatile<u16>; BUFFER_WIDTH]; BUFFER_HEIGHT];

pub struct Vga {
    buffer: &'static mut Buffer,
    x: usize,
    y: usize,
}

impl Vga {
    fn write_char(&mut self, ch: u8) {
        if ch == b'\n' {
            self.x = 0;
            self.y += 1;
        } else {
            let ch = screen_char(ch);
            let x = self.x;
            let y = self.y;
            self.buffer[y][x].write(ch);
            self.x += 1;
        }

        self.wrap();
    }

    fn wrap(&mut self) {
        if self.x >= BUFFER_WIDTH {
            self.x = 0;
            self.y += 1;
        }

        if self.y >= BUFFER_HEIGHT {
            self.y = BUFFER_HEIGHT - 1;
            self.shift_lines_up();
        }
    }

    fn shift_lines_up(&mut self) {
        for y in 0..BUFFER_HEIGHT {
            for x in 0..BUFFER_WIDTH {
                let below = if y == BUFFER_HEIGHT - 1 {
                    screen_char(b' ')
                } else {
                    self.buffer[y + 1][x].read()
                };
                self.buffer[y][x].write(below);
            }
        }
    }
}

impl fmt::Write for Vga {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        s.bytes().for_each(|ch| self.write_char(ch));
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    lazy_static! {
        static ref VGA: Mutex<Vga> = Mutex::new(Vga {
            buffer: unsafe { &mut *(VGA_ADDRESS as *mut Buffer) },
            x: 0,
            y: 0,
        });
    }

    VGA.lock().write_fmt(args).unwrap();
}

fn screen_char(ch: u8) -> u16 {
    let fg: u8 = 15;
    let bg: u8 = 0;
    let blink: u8 = 0;
    let attribute: u8 = fg | bg << 4 | blink << 7;

    (attribute as u16) << 8 | ch as u16
}
