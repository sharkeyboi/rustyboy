use super::registers;
use super::instruction::*;
use super::Memory;
pub struct CPU {
    registers: registers::Registers
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: registers::Registers::new()
        }
    }

    pub fn cycle(&mut self, memory: &mut Memory) {

        //Read one byte from memory at the current pc as an instruction.
        let mut instruction_byte = memory.read_8(self.registers.pc);

        //If instruction byte is 0xCB, it is a prefixed instruction. Handle separately
        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            instruction_byte = memory.read_8(self.registers.pc+1);
        }
        let next_pc = if let Some(instruction) = Instruction::decode(instruction_byte,prefixed) {
            self.execute(instruction, memory)
        } else {
            let description = format!("0x{}{:x}", if prefixed { "CB" } else { "" }, instruction_byte);
            panic!("Unkown instruction found for: {}", description)
        };
        self.registers.pc = next_pc;
    }


    pub fn execute(&mut self, instruction: Instruction, memory: &mut Memory) -> u16{
        match instruction {
            Instruction::ADD8(ref register) => {
                let new_value = self.add8(register);
                self.registers.a = new_value;
                self.registers.pc.wrapping_add(1)
                },
            Instruction::NOP => self.registers.pc.wrapping_add(1),
            Instruction::LD16(source, target) => {
                let source_val = match source {
                    LoadSource16::D16 => {
                        memory.read_16(self.registers.pc+1)
                    },
                    LoadSource16::Reg(ref register) => self.registers.get_16(register),
                    //_ => (panic!("{:?}",source))
                };
                match target {
                    LoadTarget16::Reg(ref register) => {
                        self.registers.set_16(register,source_val);
                    },
                    //_ => (panic!("{:?}",target))
                }
                match source {
                    LoadSource16::D16 => self.registers.pc.wrapping_add(3),
                    _ => self.registers.pc.wrapping_add(1)
                }
            },
            Instruction::LD8(source, target) => {
                let source_val = match source {
                    LoadSource8::D8 => {
                        memory.read_8(self.registers.pc+1)
                    },
                    LoadSource8::Reg(ref register) => {
                        self.registers.get_8(register)
                    },
                    LoadSource8::Address(ref register) => {
                        memory.read_8(self.registers.get_16(register))
                    }
                };
                match target {
                    LoadTarget8::Address(ref register) => {
                        memory.write_8(self.registers.get_16(register),source_val);
                    },
                    LoadTarget8::Reg(ref register) => {
                        self.registers.set_8(register,source_val);
                    },
                    LoadTarget8::AddressDec(ref register) => {
                        let curr_address = self.registers.get_16(register);
                        memory.write_8(curr_address,source_val);
                        self.registers.set_16(register,curr_address-1);
                    },
                    LoadTarget8::OffsetAddress(ref register) => {
                        let curr_address = 0xFF00 + self.registers.get_8(register) as u16;
                        memory.write_8(curr_address,source_val)
                    },
                    LoadTarget8::OffsetA8 => {
                        let curr_address = 0xFF00 + memory.read_8(self.registers.pc+1) as u16;
                        memory.write_8(curr_address,source_val)
                    }
                }
                match source {
                    LoadSource8::D8 => self.registers.pc.wrapping_add(2),
                    _ => self.registers.pc.wrapping_add(1)
                }
            },
            Instruction::XOR8(ref register) => {
                let new_value = self.xor8(register);
                self.registers.a = new_value;
                self.registers.pc.wrapping_add(1)
            },
            Instruction::BIT(source, index) => {
                match source {
                    LoadSource8::Reg(ref register) => self.bit_test(register, index),
                    _ => panic!("BIT source {:?} not implemented",source)
                }
                self.registers.pc.wrapping_add(2)
            },
            Instruction::JR(condition,source) => {
                let source_val:i8 = match source {
                    LoadSource8::D8 => memory.read_8(self.registers.pc+1) as i8,
                    _ => panic!("JR Source {:?} not implemented",source)
                };
                let should_jump = match condition {
                    JumpCondition::NZ => !self.registers.f.zero
                };
                self.jr(should_jump,source_val)
            },
            Instruction::INC8(ref register) => {
                self.registers.set_8(register,self.registers.get_8(register) + 1);
                self.registers.pc.wrapping_add(1)
            }
        }
            
        }
    
    fn jr(&mut self, should_jump:bool,value:i8) -> u16 {
        if should_jump {
            if value < 0 {
                self.registers.pc - (-value) as u16 + 2
            } else {
                self.registers.pc + value as u16 + 2
            }
        } else {
            self.registers.pc.wrapping_add(2)
        }
    }

    fn xor8(&mut self, register:&Register8) -> u8 {
        let value = self.registers.get_8(register);
        let new_value = self.registers.a ^ value;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = false;
        self.registers.f.half_carry = false;
        new_value
    }

    fn bit_test(&mut self, register: &Register8, index:u8) {
        let bit_value = (self.registers.get_8(register) >> index) & 0x01;
        self.registers.f.zero = bit_value == 0;
    }

    fn add8(&mut self, register: &Register8) -> u8 {
        let value = self.registers.get_8(&register);
        let (new_value,did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        new_value
    }
}