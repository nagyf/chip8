use std::fs::File;
use std::io;
use std::io::Read;
use std::time::Duration;

use chip8::cpu::Cpu;
use chip8::display::Display;
use chip8::keyboard::Keyboard;
use chip8::ram::Ram;

fn main() -> io::Result<()> {
    let game = load_game("games/pong_1_player.ch8")?;

    let mut ram = Ram::new();
    let mut keyboard = Keyboard::new();
    let mut display = Display::new();
    let mut cpu = Cpu::new();
    cpu.reset();
    ram.load_rom(game);

    loop {
        cpu.execute_cycle(&mut ram, &mut keyboard, &mut display);
        std::thread::sleep(Duration::from_millis(60));
    }
}

fn load_game(filename: &str) -> io::Result<[u8; 4096]> {
    let mut f = File::open(filename)?;
    let mut game = [0; 4096];
    f.read(&mut game)?;
    let mut vec = Vec::new();
    vec.extend_from_slice(&[0; 512]);
    vec.extend_from_slice(&game[0..(4096 - 512)]);

    let mut result = [0; 4096];

    for i in 0..4096 {
        result[i] = *vec.get(i).unwrap();
    }

    Ok(result)
}
