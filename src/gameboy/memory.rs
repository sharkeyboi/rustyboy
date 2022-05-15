const MEMORY_SIZE: usize = 0xFFFF;
const BOOT_LOCATION: usize = 0;
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

    pub fn read_byte(&self, address: u16) -> u8{
        self.bytes[address as usize] 
    }
}