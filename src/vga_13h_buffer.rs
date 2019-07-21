use spin::Mutex;
use volatile::Volatile;

use lazy_static::lazy_static;

pub const BUFFER_WIDTH: usize = 320;
pub const BUFFER_HEIGHT: usize = 200;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        buffer: unsafe { &mut *(0xa0000 as *mut Buffer) },
    });
}

#[repr(transparent)]
struct Buffer {
    data: [[Volatile<u8>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn read_byte(&self, x: u16, y: u16) -> u8 {
        self.buffer.data[y as usize][x as usize].read()
    }

    pub fn write_byte(&mut self, x: u16, y: u16, byte: u8) {
        self.buffer.data[y as usize][x as usize].write(byte);
    }

    pub fn xor_byte(&mut self, x: u16, y: u16, byte: u8) -> bool {
        let old_value = self.read_byte(x, y);
        self.buffer.data[y as usize][x as usize].write(old_value ^ byte);
        old_value != 0
    }
}
