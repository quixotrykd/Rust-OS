#[allow(dead_code)]
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Color {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Magenta    = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    Pink       = 13,
    Yellow     = 14,
    White      = 15,
}

#[derive(Copy, Clone)]
pub struct ColorCode(u8);

impl ColorCode {
	pub fn new(foreground: Color, background: Color) -> ColorCode {
		Self((background as u8) << 4 | (foreground as u8))
	}
}

#[derive(Copy, Clone)]
#[repr(C)]
struct ScreenChar {
	ascii_character: u8,
	color_code: ColorCode
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

pub struct Writer {
	chars: &'static mut [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
	column_position: usize,
	color_code: ColorCode,
}

impl Writer {
	pub fn write_byte(&mut self, byte: u8) {
		match byte {
			b'\n' => self.new_line(),
			_ => {
				let row = BUFFER_HEIGHT - 1;
				let col = self.column_position;

				let color_code = self.color_code;
				self.chars[row][col] = ScreenChar {
					ascii_character: byte,
					color_code
				};

				self.column_position += 1;

				if self.column_position >= BUFFER_WIDTH {
					self.new_line();
				}
			}
		}
	}

	pub fn write_string(&mut self, s: &str) {
		for byte in s.bytes() {
			match byte {
				0x20..=0x7e | b'\n' => self.write_byte(byte),
				_ => self.write_byte(0xfe)
			};
		}
	}

	fn new_line(&mut self) {
		let blank = ScreenChar {
			ascii_character: b' ',
			color_code: self.color_code
		};

		for row in 1..BUFFER_HEIGHT {
			for col in 0..BUFFER_WIDTH {
				let c = self.chars[row][col];
				self.chars[row - 1][col] = c;
				self.chars[row][col] = blank;
			}
		}

		self.column_position = 0;
	}
}

pub fn print_something() {
	let mut writer = Writer {
		chars: unsafe { &mut *(0xb8000 as *mut _) },
		column_position: 0,
		color_code: ColorCode::new(Color::LightGreen, Color::Black)
	};

	writer.write_string("Hello, World! It is I, Aidan!\n");
	writer.write_string("Hello, World! It is I, Large Brain Aidan!\n");
	writer.write_string("Hello, World! It is I, Yuuuuuge Brain Aidan!\n");
	writer.write_string("A\n");
	writer.write_string("ABA\n");
	writer.write_string("A\n");
	writer.write_byte(b'H');
    writer.write_string("ello ");
    writer.write_string("Wörld!");
}