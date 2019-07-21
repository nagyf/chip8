use crate::vga_13h_buffer;
use crate::color::Color;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const MULTIPLIER: usize = 5;

pub struct Display {
    color: Color
}

impl Display {
    /// Creates a new display with the given foreground color
    pub fn new(color: Color) -> Display {
        Display {
            color
        }
    }

    /// Clears the screen
    pub fn clear(&mut self) {
        for i in 0..WIDTH {
            for j in 0..HEIGHT {
                self.set_pixel(i, j, Color::Black);
            }
        }
    }

    /// Draws a sprite to the given x,y coordinates
    pub fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
        let mut collision = false;
        for row in 0..sprite.len() {
            let row_bytes = sprite[row];
            for column in 0..8 {
                let new_value = (row_bytes >> (7 - column)) & 0x01;
                if new_value == 1 {
                    let real_x = (x + column as usize) % WIDTH;
                    let real_y = (y + row as usize) % HEIGHT;
                    collision |= self.xor_pixel(real_x, real_y, self.color);
                }
            }
        }

        collision
    }

    fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        let mut writer = vga_13h_buffer::WRITER.lock();
        for i in 0..MULTIPLIER {
            for j in 0..MULTIPLIER {
                writer.write_byte(
                    (x * MULTIPLIER + i) as u16,
                    (y * MULTIPLIER + j) as u16,
                    color as u8);
            }
        }
    }

    fn xor_pixel(&mut self, x: usize, y: usize, color: Color) -> bool {
        let mut collision = false;
        let mut writer = vga_13h_buffer::WRITER.lock();

        // Chip8 video expects a 64x32 screen, but we have a 320x200 so each pixel must be
        // roughly 5 times bigger on our screen.
        for i in 0..MULTIPLIER {
            for j in 0..MULTIPLIER {
                collision |= writer.xor_byte(
                    (x * MULTIPLIER + i) as u16,
                    (y * MULTIPLIER + j) as u16,
                    color as u8);
            }
        }

        collision
    }
}

pub static FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];
