use std::time::Duration;

use crate::cpu::Cpu;
use crate::display::Display;
use crate::keyboard::Keyboard;
use crate::ram::Ram;

pub struct Chip8Machine {
    display: Display,
    keyboard: Keyboard,
    cpu: Cpu,
    memory: Ram,
}

impl Chip8Machine {
    pub fn new() -> Chip8Machine {
        Chip8Machine {
            display: Display::new(),
            keyboard: Keyboard::new(),
            cpu: Cpu::new(),
            memory: Ram::new(),
        }
    }

    pub fn run(&mut self, game: &[u8; 4096]) -> ! {
        self.cpu.reset();
        self.memory.load_rom(&game);

        loop {
            self.cpu.execute_cycle(&mut self.memory, &mut self.keyboard, &mut self.display);
            std::thread::sleep(Duration::from_millis(60));
        }
    }
}
