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

    pub fn read_8(&self, address: u16) -> u8{
        self.bytes[address as usize] 
    }

    pub fn read_16(&self, address:u16) -> u16 {
        let lower = self.bytes[address as usize];
        let upper = self.bytes[(address+1) as usize];
        ((upper as u16) << 8) | lower as u16
    }
}