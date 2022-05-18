const MEMORY_SIZE: usize = 0xFFFF;
const BOOT_LOCATION: usize = 0;
const ROM_BANK_0_LOCATION: (usize,usize) = (0x0000,0x3FFF);
const ROM_BANK_1_LOCATION: (usize,usize) = (0x4000,0x7FFF);
const VRAM_LOCATION: (usize, usize) = (0x8000,0x9FFF);
const EXTERNAL_RAM_LOCATION: (usize,usize) = (0xA000,0xBFFF);
const WRAM1_LOCATION:(usize,usize) = (0xC000,0xCFFF);
const WRAM2_LOCATION:(usize,usize) = (0xD000,0xDFFF);
const ECHO_RAM_LOCATION:(usize,usize) = (0xC000,0xDDFF);
const SPRITE_TABLE_LOCATION:(usize,usize) = (0xFE00,0xFE9F);
const NOT_USABLE_LOCATION:(usize,usize) = (0xFEA0,0xFEFF);
const IO_REGISTERS_LOCATION:(usize,usize) = (0xFF00,0xFF7F);
const HRAM_LOCATION:(usize,usize) = (0xFF80,0xFFFE);
const IE_LOCATION:(usize,usize) = (0xFFFF,0xFFFF); 

pub enum MemoryLocation {
    RomBank0,
    RomBank1,
    VideoRAM,
    ExternalRAM,
    WorkRAM1,
    WorkRAM2,
    EchoRam,
    SpriteTable,
    NotUsable,
    IORegisters,
    HighRam,
    InterruptEnableRegister
}

pub struct Memory {
    bytes: [u8; MEMORY_SIZE]
}

impl Memory {
    pub fn new() -> Self {
        Self {
            bytes: [0;MEMORY_SIZE]
        }
    }



    pub fn load_boot_rom(&mut self,data: & Vec<u8>) {
        for (i,elem) in data.iter().enumerate() {
            self.bytes[BOOT_LOCATION+i] = *elem;
        }
    }

    pub fn read_8(&self, address: u16) -> u8{
        self.bytes[address as usize] 
    }

    pub fn read_16(&self, address:u16) -> u16 {
        let lower = self.bytes[address as usize];
        let upper = self.bytes[(address+1) as usize];
        ((upper as u16) << 8) | lower as u16
    }

    pub fn write_8(&mut self, address:u16,value:u8) {
        self.bytes[address as usize] = value;
    }
}