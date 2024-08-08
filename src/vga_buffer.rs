use core::fmt;

use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new());
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Colour {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColourCode(u8);

impl ColourCode {
    fn new(foreground: Colour, background: Colour) -> ColourCode {
        ColourCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    colour_code: ColourCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_pos: usize,
    colour_code: ColourCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn new() -> Self {
        let mut writer = Self {
            column_pos: 0,
            colour_code: ColourCode::new(Colour::Red, Colour::Black),
            buffer: unsafe { &mut *(0xB8000 as *mut Buffer) },
        };

        writer.clear();

        writer
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_pos >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;

                self.buffer.chars[row][self.column_pos].write(ScreenChar {
                    ascii_character: byte,
                    colour_code: self.colour_code,
                });

                self.column_pos += 1;
            }
        }
    }

    pub fn write_str(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // Printable ASCII byte or a newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // Backspace
                8 => self.backspace(),
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let chr = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(chr);
            }
        }

        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_pos = 0;
    }

    fn backspace(&mut self) {
        if self.column_pos > 0 {
            self.buffer.chars[BUFFER_HEIGHT - 1][self.column_pos - 1].write(ScreenChar {
                ascii_character: b' ',
                colour_code: self.colour_code,
            });

            self.column_pos -= 1;
        } else {
            self.copy_all_rows_down(BUFFER_HEIGHT - 1);
            self.column_pos = self.last_non_empty_char(BUFFER_HEIGHT - 1);
        }
    }

    fn copy_all_rows_down(&mut self, to: usize) {
        for row in (1..=to).rev() {
            for col in 0..BUFFER_WIDTH {
                self.buffer.chars[row][col] = self.buffer.chars[row - 1][col].clone();
            }
        }
    }

    fn last_non_empty_char(&self, row: usize) -> usize {
        for col in (0..BUFFER_WIDTH).rev() {
            if self.buffer.chars[row][col].read().ascii_character != b' ' {
                return col + 1;
            }
        }

        0
    }

    fn clear_row(&mut self, row: usize) {
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(ScreenChar {
                ascii_character: b' ',
                colour_code: self.colour_code,
            });
        }
    }

    fn clear(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: b' ',
                    colour_code: self.colour_code,
                });
            }
        }
    }
}

impl Default for Writer {
    fn default() -> Self {
        Self::new()
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str(s);

        Ok(())
    }
}
