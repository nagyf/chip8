extern crate rand;

use rand::Rng;

use crate::display::Display;
use crate::keyboard::Keyboard;
use crate::ram::Ram;

///
/// CHIP-8 memory map
///
/// +---------------+= 0xFFF (4095) End of Chip-8 RAM
/// |               |
/// |               |
/// |               |
/// |               |
/// |               |
/// | 0x200 to 0xFFF|
/// |     Chip-8    |
/// | Program / Data|
/// |     Space     |
/// |               |
/// |               |
/// |               |
/// +- - - - - - - -+= 0x600 (1536) Start of ETI 660 Chip-8 programs
/// |               |
/// |               |
/// |               |
/// +---------------+= 0x200 (512) Start of most Chip-8 programs
/// | 0x000 to 0x1FF|
/// | Reserved for  |
/// |  interpreter  |
/// +---------------+= 0x000 (0) Start of Chip-8 RAM
///
pub struct Cpu {
    /// index register
    pub i: u16,

    /// program counter
    pub pc: u16,

    /// registers usually referred to as Vx, where x is a hexadecimal digit (0 through F)
    pub v: [u8; 16],

    /// Stack: used to store the address that the interpreter should return to when finished with a subroutine
    /// Chip-8 allows for up to 16 levels of nested subroutines.
    pub stack: [u16; 16],

    /// stack pointer
    pub sp: u8,

    /// Delay timer
    pub dt: u8,

    /// Sound timer
    pub st: u8,
}

fn read_word(memory: [u8; 4096], index: u16) -> u16 {
    let i = index as usize;
    (memory[i] as u16) << 8 | (memory[i + 1] as u16)
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            i: 0,
            pc: 0x200,
            v: [0; 16],
            stack: [0; 16],
            sp: 0,
            dt: 0,
            st: 0,
        }
    }

    pub fn reset(&mut self) {
        self.i = 0;
        self.pc = 0x200;
        self.v = [0; 16];
        self.stack = [0; 16];
        self.sp = 0;
        self.dt = 0;
        self.st = 0;
    }

    pub fn execute_cycle(&mut self, ram: &mut Ram, keyboard: &mut Keyboard, display: &mut Display) {
        let opcode = read_word(ram.memory, self.pc);
        self.pc += 2;
        self.process_opcode(opcode, ram, keyboard, display);
    }

    fn process_opcode(&mut self, opcode: u16, ram: &mut Ram, keyboard: &mut Keyboard, display: &mut Display) {
        println!("{:x}", opcode);
        match opcode {
            0x00E0 => {
                // 00E0 - CLS
                // Clear the display.
                display.clear();
            }
            0x00EE => {
                // 00EE - RET
                // Return from a subroutine.
                // The interpreter sets the program counter to the address at the top of the stack,
                // then subtracts 1 from the stack pointer.
                self.pc = self.stack[self.sp as usize];
                self.sp -= 1;
            }
            0x1000...0x1FFF => {
                // 1nnn - JP addr
                // 1nnn - JP addr - Jump to location nnn.
                // The interpreter sets the program counter to nnn.
                self.pc = opcode & 0x0FFF;
            }
            0x2000...0x2FFF => {
                // 2nnn - CALL addr
                // Call subroutine at nnn.
                // The interpreter increments the stack pointer, then puts the current PC on the top of the stack.
                // The PC is then set to nnn.
                self.sp += 1;
                self.stack[self.sp as usize] = self.pc;
                self.pc = opcode & 0x0FFF;
            }
            0x3000...0x3FFF => {
                // 3xkk - SE Vx, byte
                // Skip next instruction if Vx = kk.
                // The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
                let x = (opcode & 0x0F00) >> 8;
                let value = (opcode & 0x00FF) as u8;
                if self.v[x as usize] == value {
                    self.pc += 2;
                }
            }
            0x4000...0x4FFF => {
                // 4xkk - SNE Vx, byte
                // Skip next instruction if Vx != kk.
                // The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
                let x = (opcode & 0x0F00) >> 8;
                let value = (opcode & 0x00FF) as u8;
                if self.v[x as usize] != value {
                    self.pc += 2;
                }
            }
            0x5000...0x5FFF => {
                // 5xy0 - SE Vx, Vy
                // Skip next instruction if Vx = Vy.
                // The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
                let x = (opcode & 0x0F00) >> 8;
                let y = (opcode & 0x00F0) >> 4;
                if self.v[x as usize] == self.v[y as usize] {
                    self.pc += 2;
                }
            }
            0x6000...0x6FFF => {
                // 6xkk - LD Vx, byte
                // Set Vx = kk.
                // The interpreter puts the value kk into register Vx.
                let x = (opcode & 0x0F00) >> 8;
                let kk = (opcode & 0x00FF) as u8;
                self.v[x as usize] = kk;
            }
            0x7000...0x7FFF => {
                // 7xkk - ADD Vx, byte
                // Set Vx = Vx + kk.
                // Adds the value kk to the value of register Vx, then stores the result in Vx.
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let kk = (opcode & 0x00FF) as u8;
                self.v[x] = self.v[x].wrapping_add(kk);
            }
            0x8000...0x8FF0 => {
                // 8xy0 - LD Vx, Vy
                // Set Vx = Vy.
                // Stores the value of register Vy in register Vx.
                let x = (opcode & 0x0F00) >> 8;
                let y = (opcode & 0x00F0) >> 4;
                self.v[x as usize] = self.v[y as usize];
            }
            0x8001...0x8FF1 => {
                // 8xy1 - OR Vx, Vy
                // Set Vx = Vx OR Vy.
                //
                // Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx.
                // A bitwise OR compares the corresponding bits from two values, and if either bit is 1,
                // then the same bit in the result is also 1. Otherwise, it is 0.
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;
                self.v[x] = self.v[x] | self.v[y];
            }
            0x8002...0x8FF2 => {
                // 8xy2 - AND Vx, Vy
                // Set Vx = Vx AND Vy.
                //
                // Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx.
                // A bitwise AND compares the corrseponding bits from two values, and if both bits are 1,
                // then the same bit in the result is also 1. Otherwise, it is 0.
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;
                self.v[x] = self.v[x] & self.v[y];
            }
            0x8003...0x8FF3 => {
                // 8xy3 - XOR Vx, Vy
                // Set Vx = Vx XOR Vy.
                //
                // Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx.
                // An exclusive OR compares the corrseponding bits from two values, and if the bits are not both the same,
                // then the corresponding bit in the result is set to 1. Otherwise, it is 0.
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;
                self.v[x] = self.v[x] ^ self.v[y];
            }
            0x8004...0x8FF4 => {
                // 8xy4 - ADD Vx, Vy
                // Set Vx = Vx + Vy, set VF = carry.
                //
                // The values of Vx and Vy are added together.
                // If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0.
                // Only the lowest 8 bits of the result are kept, and stored in Vx.
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;
                let result = self.v[x] as u16 + self.v[y] as u16;
                self.v[0xF as usize] = if result > 255 { 1 } else { 0 };
                self.v[x] = self.v[x].wrapping_add(self.v[y]);
            }
            0x8005...0x8FF5 => {
                // 8xy5 - SUB Vx, Vy
                // Set Vx = Vx - Vy, set VF = NOT borrow.
                //
                // If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;
                let xx = self.v[x];
                let yy = self.v[y];

                self.v[0xF as usize] = if xx > yy { 1 } else { 0 };
                self.v[x] = xx.wrapping_sub(yy);
            }
            0x8006...0x8FF6 => {
                // 8xy6 - SHR Vx {, Vy}
                // Set Vx = Vx SHR 1.
                //
                // If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
                let x = ((opcode & 0x0F00) >> 8) as usize;
                self.v[0xF as usize] = if self.v[x] & 0x01 > 0 { 1 } else { 0 };
                self.v[x] = self.v[x] >> 1;
            }
            0x8007...0x8FF7 => {
                // 8xy7 - SUBN Vx, Vy
                // Set Vx = Vy - Vx, set VF = NOT borrow.
                //
                // If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;
                let xx = self.v[x];
                let yy = self.v[y];

                self.v[0xF as usize] = if yy > xx { 1 } else { 0 };
                self.v[x] = yy.wrapping_sub(xx);
            }
            0x800E...0x8FFE => {
                // 8xyE - SHL Vx {, Vy}
                // Set Vx = Vx SHL 1.
                //
                // If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
                let x = ((opcode & 0x0F00) >> 8) as usize;

                self.v[0xF as usize] = if self.v[x] & 0x80 > 0 { 1 } else { 0 };
                self.v[x] = self.v[x] << 1;
            }
            0x9000...0x9FF0 => {
                // 9xy0 - SNE Vx, Vy
                // Skip next instruction if Vx != Vy.
                //
                // The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;

                if self.v[x] != self.v[y] {
                    self.pc += 2;
                }
            }
            0xA000...0xAFFF => {
                // Annn - LD I, addr
                // Set I = nnn.
                //
                // The value of register I is set to nnn.
                self.i = opcode & 0x0FFF;
            }
            0xB000...0xBFFF => {
                // Bnnn - JP V0, addr
                // Jump to location nnn + V0.
                //
                // The program counter is set to nnn plus the value of V0.
                let delta = opcode & 0x0FFF;
                self.pc = (self.v[0] as u16).wrapping_add(delta);
            }
            0xC000...0xCFFF => {
                // Cxkk - RND Vx, byte
                // Set Vx = random byte AND kk.
                //
                // The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk.
                // The results are stored in Vx. See instruction 8xy2 for more information on AND.
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let kk = (opcode & 0x00FF) as u8;
                let random: u8 = rand::thread_rng().gen_range(0, 255);
                self.v[x] = kk & random;
            }
            0xD000...0xDFFF => {
                // Dxyn - DRW Vx, Vy, nibble
                // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
                //
                // The interpreter reads n bytes from memory, starting at the address stored in I.
                // These bytes are then displayed as sprites on screen at coordinates (Vx, Vy).
                // Sprites are XORed onto the existing screen. If this causes any pixels to be erased,
                // VF is set to 1, otherwise it is set to 0. If the sprite is positioned so part of
                // it is outside the coordinates of the display, it wraps around to the opposite side of the screen.
                // See instruction 8xy3 for more information on XOR, and section 2.4, Display, for more information on the Chip-8 screen and sprites.
                let x = ((opcode & 0x0F00) >> 8) as u8;
                let y = ((opcode & 0x00F0) >> 4) as u8;
                let n = (opcode & 0x000F) as u16;
                let from = self.i as usize;
                let to = (self.i + n) as usize;
                let mut bytes = Vec::new();
                bytes.extend_from_slice(&ram.memory[from..to]);
                display.draw(x, y, &bytes);
            }
            0xE09E...0xEF9E => {
                // Ex9E - SKP Vx
                // Skip next instruction if key with the value of Vx is pressed.
                //
                // Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, PC is increased by 2.
                let x = ((opcode & 0x0F00) >> 8) as usize;
                if keyboard.is_pressed(self.v[x]) {
                    self.pc += 2;
                }
            }
            0xE0A1...0xEFA1 => {
                // ExA1 - SKNP Vx
                // Skip next instruction if key with the value of Vx is not pressed.
                //
                // Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.
                let x = ((opcode & 0x0F00) >> 8) as usize;
                if keyboard.is_released(self.v[x]) {
                    self.pc += 2;
                }
            }
            0xF007...0xFF07 => {
                // Fx07 - LD Vx, DT
                // Set Vx = delay timer value.
                //
                // The value of DT is placed into Vx.
                let x = ((opcode & 0x0F00) >> 8) as usize;
                self.v[x] = self.dt;
            }
            0xF00A...0xFF0A => {
                // Fx0A - LD Vx, K
                // Wait for a key press, store the value of the key in Vx.
                //
                // All execution stops until a key is pressed, then the value of that key is stored in Vx.
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let key_pressed = keyboard.wait_key();
                self.v[x] = key_pressed;
            }
            0xF015...0xFF15 => {
                // Fx15 - LD DT, Vx
                // Set delay timer = Vx.
                //
                // DT is set equal to the value of Vx.
                let x = ((opcode & 0x0F00) >> 8) as usize;
                self.dt = self.v[x];
            }
            0xF018...0xFF18 => {
                // Fx18 - LD ST, Vx
                // Set sound timer = Vx.
                //
                // ST is set equal to the value of Vx.
                let x = ((opcode & 0x0F00) >> 8) as usize;
                self.st = self.v[x];
            }
            0xF01E...0xFF1E => {
                // Fx1E - ADD I, Vx
                // Set I = I + Vx.
                //
                // The values of I and Vx are added, and the results are stored in I.
                let x = ((opcode & 0x0F00) >> 8) as usize;
                self.i += self.v[x] as u16;
            }
            0xF029...0xFF29 => {
                // Fx29 - LD F, Vx
                // Set I = location of sprite for digit Vx.
                //
                // The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx.
                // TODO
            }
            0xF033...0xFF33 => {
                // Fx33 - LD B, Vx
                // Store BCD representation of Vx in memory locations I, I+1, and I+2.
                //
                // The interpreter takes the decimal value of Vx, and places the hundreds digit
                // in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let i = self.i as usize;
                let mut num = self.v[x];

                ram.memory[i] = num / 100;
                num = num % 100;

                ram.memory[i + 1] = num / 10;
                num = num % 10;

                ram.memory[i + 2] = num;
            }
            0xF055...0xFF55 => {
                // Fx55 - LD [I], Vx
                // Store registers V0 through Vx in memory starting at location I.
                //
                // The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.
                let x = ((opcode & 0x0F00) >> 8) as usize;
                for i in 0..x {
                    ram.memory[self.i as usize + i] = self.v[i];
                }
            }
            0xF065...0xFF65 => {
                // Fx65 - LD Vx, [I]
                // Read registers V0 through Vx from memory starting at location I.
                //
                // The interpreter reads values from memory starting at location I into registers V0 through Vx.
                let x = ((opcode & 0x0F00) >> 8) as usize;
                for i in 0..x {
                    self.v[i] = ram.memory[self.i as usize + i];
                }
            }

            _ => {
                panic!("Unknown opcode: {:x}", opcode);
            }
        }
    }
}
