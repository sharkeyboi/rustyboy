//Instruction sets, in order of appearance during development
#[derive(Debug)]
pub enum Instruction {
    ADD8(Register8),
    NOP,
    LD16(LoadSource16, LoadTarget16),
    XOR8(Register8),
    LD8(LoadSource8,LoadTarget8),
    BIT(LoadSource8,u8),
    JR(JumpCondition,LoadSource8),
    INC8(Register8),
    CALL(CallCondition),
    PUSH(Register16), // TODO: Verify that this should be a normal stack push, EG. Decrement SP, save upper byte, decrement again, save lower byte.
    RET,
    RL(LoadSource8),
    INC16(Register16),
    POP(Register16),
    DEC8(Register8),
    CP(LoadSource8)
}

#[derive(Debug)]
pub enum CallCondition {
    None
}

#[derive(Debug)]
pub enum LoadSource8 {
    Reg(Register8),Address(Register16),D8
}

#[derive(Debug)]
pub enum LoadTarget8 {
    Reg(Register8), Address(Register16), AddressDec(Register16), AddressInc(Register16),OffsetAddress(Register8), OffsetA8, AddressD8
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
pub enum JumpCondition {
    NZ, Z
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

    //Translate opcode into instructions using above enums
    pub fn decode(byte:u8, prefixed:bool) -> Option<Instruction> {
        if prefixed {
            Instruction::decode_prefixed(byte)
        }
        else {
            Instruction::decode_not_prefixed(byte)
        }
    }

    //Prefixed opcodes
    fn decode_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x7C => Some(Instruction::BIT(LoadSource8::Reg(Register8::H),7)), // BIT 7 H
            0x11 => Some(Instruction::RL(LoadSource8::Reg(Register8::C))), // RL C
            _ => None
        }
    }

    //Non prefixed opcodes
    fn decode_not_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => Some(Instruction::NOP), //NOP
            0x31 => Some(Instruction::LD16(LoadSource16::D16,LoadTarget16::Reg(Register16::SP))), //LD SP D16
            0xAF => Some(Instruction::XOR8(Register8::A)), //XOR A
            0x21 => Some(Instruction::LD16(LoadSource16::D16, LoadTarget16::Reg(Register16::HL))), // LD HL D16
            0x32 => Some(Instruction::LD8(LoadSource8::Reg(Register8::A),LoadTarget8::AddressDec(Register16::HL))), // LD (HL) A
            0x20 => Some(Instruction::JR(JumpCondition::NZ,LoadSource8::D8)), // JR NZ D8
            0x0E => Some(Instruction::LD8(LoadSource8::D8,LoadTarget8::Reg(Register8::C))), //LD C D8
            0x3E => Some(Instruction::LD8(LoadSource8::D8,LoadTarget8::Reg(Register8::A))), //LD A D8
            0xE2 => Some(Instruction::LD8(LoadSource8::Reg(Register8::A),LoadTarget8::OffsetAddress(Register8::C))), // LD (C) A
            0x0C => Some(Instruction::INC8(Register8::C)), // INC C
            0x77 => Some(Instruction::LD8(LoadSource8::Reg(Register8::A),LoadTarget8::Address(Register16::HL))), // LD (HL) A
            0xe0 => Some(Instruction::LD8(LoadSource8::Reg(Register8::A),LoadTarget8::OffsetA8)), // LD (a8) A
            0x11 => Some(Instruction::LD16(LoadSource16::D16,LoadTarget16::Reg(Register16::DE))), // LD DE d16
            0x1A => Some(Instruction::LD8(LoadSource8::Address(Register16::DE),LoadTarget8::Reg(Register8::A))), // LD A (DE)
            0xCD => Some(Instruction::CALL(CallCondition::None)), // Call a16
            0x4F => Some(Instruction::LD8(LoadSource8::Reg(Register8::A),LoadTarget8::Reg(Register8::C))), // LD C A
            0x06 => Some(Instruction::LD8(LoadSource8::D8,LoadTarget8::Reg(Register8::B))), // LD B d8
            0xc5 => Some(Instruction::PUSH(Register16::BC)), // PUSH BC
            0x17 => Some(Instruction::RL(LoadSource8::Reg(Register8::A))), // RL A
            0x22 => Some(Instruction::LD8(LoadSource8::Reg(Register8::A),LoadTarget8::AddressInc(Register16::HL))), // LD (HL+) A
            0xC1 => Some(Instruction::POP(Register16::BC)), // POP BC
            0x05 => Some(Instruction::DEC8(Register8::B)), // DEC B
            0x23 => Some(Instruction::INC16(Register16::HL)), // INC HL
            0xC9 => Some(Instruction::RET), // RET
            0x13 => Some(Instruction::INC16(Register16::DE)), // INC DE
            0x7B => Some(Instruction::LD8(LoadSource8::Reg(Register8::E),LoadTarget8::Reg(Register8::A))), // LD A E
            0xFE => Some(Instruction::CP(LoadSource8::D8)), // CP d8
            0xEA => Some(Instruction::LD8(LoadSource8::Reg(Register8::A),LoadTarget8::AddressD8)), // LD (a16) A
            0x3D => Some(Instruction::DEC8(Register8::A)), // DEC A
            0x28 => Some(Instruction::JR(JumpCondition::Z,LoadSource8::D8)), // JR Z s8
            0x67 => Some(Instruction::LD8(LoadSource8::Reg(Register8::A),LoadTarget8::Reg(Register8::H))), // LD H A
            0x57 => Some(Instruction::LD8(LoadSource8::Reg(Register8::A),LoadTarget8::Reg(Register8::D))), // LD D A
            _ => None
        }
    }
}