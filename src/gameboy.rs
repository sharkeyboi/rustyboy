use std::io::Read;
use std::fs::File;
use std::io;

use cpu::CPU;
use memory::Memory;

mod cpu;
mod registers;
mod memory;
mod instruction;

pub struct Gameboy {
    cpu: CPU,
    memory: Memory
}

impl Gameboy {
    pub fn new() -> Self {
        Self {
            cpu: CPU::new(),
            memory: Memory::new()
        }
    }

    pub fn load_boot_rom(&mut self, path: &str) -> io::Result<()> {
        let mut rom_file = File::open(path)?;
        let mut rom_data = Vec::new();

        let rom_size = rom_file.read_to_end(&mut rom_data)?;
        self.memory.load_boot_rom(&rom_data);
        Ok(())
    }

    pub fn run(&mut self) {
        loop {
            self.cpu.cycle(&mut self.memory);
        }
    }

}