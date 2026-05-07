use core::fmt;
use core::fmt::Write;
use spin::Mutex;
use spin::Lazy;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
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
struct ColorCode(u8);

impl ColorCode {
    const fn new(foreground: Color, background: Color) -> Self {
        Self((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn new() -> Self {
        Self {
            column_position: 0,
            color_code: ColorCode::new(Color::Yellow, Color::Black),
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        }
    }

    pub fn set_color(&mut self, foreground: Color, background: Color) {
        self.color_code = ColorCode::new(foreground, background);
    }

    pub fn clear_screen(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            self.clear_row(row);
        }
        self.column_position = 0;
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                b'\n' => self.new_line(),
                0x20..=0x7e => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                unsafe {
                    core::ptr::write_volatile(
                        &mut self.buffer.chars[row][col],
                        ScreenChar {
                            ascii_character: byte,
                            color_code: self.color_code,
                        },
                    );
                }

                self.column_position += 1;
            }
        }
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };

        for col in 0..BUFFER_WIDTH {
            unsafe {
                core::ptr::write_volatile(
                    &mut self.buffer.chars[row][col],
                    blank,
                );
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = unsafe {
                    core::ptr::read_volatile(
                        &self.buffer.chars[row][col]
                    )
                };

                unsafe {
                    core::ptr::write_volatile(
                        &mut self.buffer.chars[row - 1][col],
                        character,
                    );
                }
            }
        }

        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

pub static WRITER: Lazy<Mutex<Writer>> = Lazy::new(|| Mutex::new(Writer::new()));

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    WRITER.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::vga_buffer::_print(format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! println {
    () => ({
        $crate::print!("\n");
    });

    ($fmt:expr) => ({
        $crate::print!(concat!($fmt, "\n"));
    });

    ($fmt:expr, $($arg:tt)*) => ({
        $crate::print!(
            concat!($fmt, "\n"),
            $($arg)*
        );
    });
}

// Example usage
pub fn print_something() {
    let mut writer = WRITER.lock();

    writer.clear_screen();

    writer.set_color(Color::LightGreen, Color::Black);
    writeln!(writer, "Hello, World!").unwrap();

    writeln!(
        writer,
        "VGA Buffer driver initialized successfully."
    )
    .unwrap();

    writer.set_color(Color::Pink, Color::Black);

    writeln!(writer, "Color support works!").unwrap();
}