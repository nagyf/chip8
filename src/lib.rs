#![no_std]

pub mod color;
pub mod vga_13h_buffer;
#[macro_use]
pub mod vga_text_buffer;
pub mod chip8;
pub mod cpu;
pub mod display;
pub mod keyboard;
pub mod ram;

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
