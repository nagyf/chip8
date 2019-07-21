use crate::cpu::Cpu;
use crate::display::{Display, FONT};
use crate::keyboard::Keyboard;
use crate::ram::Ram;
use crate::color::Color;

pub struct Chip8Machine {
    display: Display,
    keyboard: Keyboard,
    cpu: Cpu,
    memory: Ram,
}

impl Chip8Machine {
    pub fn new() -> Chip8Machine {
        Chip8Machine {
            display: Display::new(Color::White),
            keyboard: Keyboard::new(),
            cpu: Cpu::new(),
            memory: Ram::new(),
        }
    }

    pub fn run(&mut self, game: &[u8]) -> ! {
        self.cpu.reset();
        let mut memory = [0; 4096];
        // Load the game's ROM into memory
        for i in 0..game.len() {
            memory[self.cpu.pc as usize + i] = game[i];
        }

        // Load the font into memory, at the very beginning
        for i in 0..80 {
            memory[i] = FONT[i];
        }

        self.memory.load_rom(&memory);

        loop {
            self.cpu.execute_cycle(&mut self.memory, &mut self.keyboard, &mut self.display);
        }
    }
}
