use std::fs::File;
use std::io;
use std::io::Read;

use chip8::chip8::Chip8Machine;

fn main() -> io::Result<()> {
    let game = load_game("games/pong_1_player.ch8")?;

    let mut machine = Chip8Machine::new();
    machine.run(&game);
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
