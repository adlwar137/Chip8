pub mod display;
mod font;
mod processor;

use core::panic;

use display::Screen;
use crate::font::FONT;
use processor::Cpu;

pub struct Chip8 {
    processor: Cpu,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let mut processor = Cpu::new();
        //load fonts
        for byte in 0..FONT.len() {
            processor.edit_memory(byte, FONT[byte]);
        }

        Chip8 { processor }
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        //sanity check to make sure rom isn't bigger than memory

        const OFFSET: usize = 0x200;

        if rom.len() > (0xFFF - 0x200) {
            panic!("Rom is bigger than amount of memory!");
        }

        for byte in 0..(rom.len()) {
            self.processor.edit_memory(byte + OFFSET, rom[byte]);
        }
    }

    pub fn tick(&mut self) {
        self.processor.process_instruction();
    }
}