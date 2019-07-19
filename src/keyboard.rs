pub struct Keyboard {}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {}
    }

    pub fn is_pressed(&self, _key: u8) -> bool {
        // TODO
        false
    }

    pub fn is_released(&self, _key: u8) -> bool {
        // TODO
        true
    }

    pub fn wait_key(&self) -> u8 {
        // TODO
        0x00
    }
}
