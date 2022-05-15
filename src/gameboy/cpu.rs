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
        let mut instruction_byte = memory.read_byte(self.registers.pc);

        //If instruction byte is 0xCB, it is a prefixed instruction. Handle separately
        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            instruction_byte = memory.read_byte(self.registers.pc+1);
        }
    }


    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(register) => {
                let new_value = self.add(register);
                self.registers.a = new_value;
                }
            }
        }

    fn add(&mut self, register: String) -> u8 {
        let mut value = 0;
        match register {
            _ => println!("ERROR INVALID REGISTER")
        }
        let (new_value,did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        // Half Carry is set if adding the lower nibbles of the value and register A
        // together result in a value bigger than 0xF. If the result is larger than 0xF
        // than the addition caused a carry from the lower nibble to the upper nibble.
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        new_value
    }
}