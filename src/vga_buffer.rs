use lazy_static::lazy_static;
use spin::Mutex;

/*
* In order to limit the amount of unsafe code in our final kernel
* we want to statically initialize a writer object which can then
* be globally referenced.
*
* However, we want to call ColorCode::new which is a non-const
* function! lazy_static provides a macro which initialzes
* this value at runtim rather than compile time (as other statics are)
*
* The way this works behind the scenes is that lazy_static creates unique
* types for each "variable" that is defined. This type then implements Deref<T>
* where T is the "type" that we define (in this case Mutex<Writer>)
*
* We could create a mutable Writer but because this could eventually get
* accessed concurrently, it's safer to wrap it in a Mutex to provide interior
* mutable that "safe" under concurrent access.
*
* Since we don't have access to the standard library, we are using
* `spin::Mutex` which can be compiled independently of `std`.
*
*/
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        row_position: 0,
        color_code: ColorCode::new(Color::White, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut ScreenBuffer) },
    });
}

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
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
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

use core::fmt;

/*
* Generally we will write to but never read from the screen buffer.
* Because of this, the Rust compiler may optimize away our writes!
* We need these writes because of their side effects.
* The `volatile` crate enables us to wrap any type which implements
* `Copy` in a `Volatile` wrapper that forces any writes to that type
* to use `std::ptr::read_volatile`(https://doc.rust-lang.org/std/ptr/fn.read_volatile.html)
* to ensure that those writes are never optimized away
*/
use volatile::Volatile;

#[repr(transparent)]
struct ScreenBuffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    row_position: usize,
    color_code: ColorCode,
    buffer: &'static mut ScreenBuffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                let row = self.row_position;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }
    pub fn write_string(&mut self, s: impl AsRef<str>) {
        for byte in s.as_ref().bytes() {
            match byte {
                // From space through to ~
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    fn new_line(&mut self) {
        if self.row_position < BUFFER_HEIGHT - 1 {
            self.column_position = 0;
            self.row_position += 1;
            return;
        }

        // copy everything one row up
        for row in 1..BUFFER_HEIGHT - 1 {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
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

// These are just copied from `std::macros::println` but using our own Writer
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
    WRITER.lock().write_fmt(args).unwrap();
}
