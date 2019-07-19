pub struct Ram {
    /// 4 kb of memory
    pub memory: [u8; 4096]
}

impl Ram {
    pub fn new() -> Ram {
        Ram {
            memory: [0; 4096],
        }
    }

    pub fn load_rom(&mut self, rom: [u8; 4096]) {
        self.memory = rom;
    }
}
