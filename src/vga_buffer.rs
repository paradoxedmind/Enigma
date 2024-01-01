use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile; //To mark our read/write as volatile(means they have side effect and should not be optimized) // Basic Mutex where thread simply try to lock it again and again in loop, burning CPU time until  mutex is free again

lazy_static! { // This Initalize itself when accessed first time instead of compiled time
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        col_position: 0,
        color_code: ColorCode::new(Color::LightRed,Color::Black),
        buffer: unsafe { &mut *( 0xb8000 as *mut Buffer )  },
    });
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)] // To expilcitly specify color no use c-like enum. Stores each enum as u8
pub enum Color {
    // VGA Colors
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
#[repr(transparent)] // To get Same Memory layout as u8
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> Self {
        // Color Format according to VGA attribute
        Self((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)] // To guarantee that field are laid exactly like C so field ordering remain correct so
           // that it directly maps to VGA Buffer cell
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

// VGA Default Text Mode (80x25)
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)] // To have same memory layout as single field
struct Buffer {
    // VGA Buffer (at 0xb8000)
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    col_position: usize, // Current Position in last row
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // VGA Supports Code Page 437 character set
                // Printable ASCII Byte b/w ` ` to `~` or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // Non Printable ASCII Range, we print `■`
                _ => self.write_byte(0xfe),
            }
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.col_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1; // Because writing on last line
                let col = self.col_position;

                let color_code = self.color_code;

                // Compiler will not optimize this write
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });

                self.col_position += 1;
            }
        }
    }

    // write current character to line above it and topline gets deleted.
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.col_position = 0;
    }

    // Clear row by overwritting space character
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };

        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

// To Support write! and writeln! formatting macros
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

// Macros for printing to VGA Buffer
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

// Prints given formatted string to VGA text buffer:w
// through global `WRITER` instance
#[doc(hidden)] // Hide it from generated documentation as it is private implementation detail
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        // Runs closure code in interrupt-free environment, avoiding the deadlock
        WRITER.lock().write_fmt(args).unwrap();
    })
}

#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}

#[test_case]
fn test_println_output() {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    let s = "Some test string that fits on a single line";
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writeln!(writer, "\n{}", s).expect("writeln failed");
        for (i, c) in s.chars().enumerate() {
            let screen_char = writer.buffer.chars[BUFFER_HEIGHT - 2][i].read();
            assert_eq!(char::from(screen_char.ascii_character), c);
        }
    });
}
