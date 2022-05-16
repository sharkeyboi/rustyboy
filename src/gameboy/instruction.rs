pub enum Instruction {
    ADD8(Register8),
    NOP,
    LD16(LoadSource16, LoadTarget16),
    XOR8(Register8)
}

pub enum LoadSource8 {
    Reg(Register8),D8
}

pub enum LoadTarget8 {
    Reg(Register8), HLI
}

#[derive(Debug)]
pub enum LoadSource16 {
    Reg(Register16),D16
}

#[derive(Debug)]
pub enum LoadTarget16 {
    Reg(Register16)
}

#[derive(Debug)]
pub enum Register8 {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L
}

#[derive(Debug)]
pub enum Register16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC
}

impl Instruction {
    pub fn decode(byte:u8, prefixed:bool) -> Option<Instruction> {
        if prefixed {
            Instruction::decode_prefixed(byte)
        }
        else {
            Instruction::decode_not_prefixed(byte)
        }
    }

    fn decode_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            _ => None
        }
    }
    fn decode_not_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => Some(Instruction::NOP),
            0x31 => Some(Instruction::LD16(LoadSource16::D16,LoadTarget16::Reg(Register16::SP))),
            0xAF => Some(Instruction::XOR8(Register8::A)),
            _ => None
        }
    }
}