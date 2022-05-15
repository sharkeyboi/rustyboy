const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

use super::instruction::Register8;
use super::instruction::Register16;

pub struct FlagsRegister {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool
}

impl std::convert::From<&FlagsRegister> for u8  {
    fn from(flag: &FlagsRegister) -> u8 {
        (if flag.zero       { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION |
        (if flag.subtract   { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION |
        (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION |
        (if flag.carry      { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry
        }
    }
}

pub struct Registers {
    pub a:u8,
    pub b:u8,
    pub c:u8,
    pub d:u8,
    pub e:u8,
    pub f:FlagsRegister,
    pub h:u8,
    pub l:u8,
    pub sp: u16,
    pub pc: u16
}

impl Registers {
    pub fn new() -> Self {
        Self {
            a:0x01,
            b:0x00,
            c:0x13,
            d:0x00,
            e:0xD8,
            f:0xB0.into(),
            h:0x01,
            l:0x4D,
            sp: 0xFFFE,
            pc: 0
        }
    }
    pub fn get_8(&self,name:Register8) -> u8{
        match name {
            Register8::A => self.a,
            Register8::B => self.b,
            Register8::C => self.c,
            Register8::D => self.d,
            Register8::E => self.e,
            Register8::F => u8::from(&self.f),
            Register8::H => self.h,
            Register8::L => self.l
        }
    }

    pub fn set_8(&mut self,name:Register8, value:u8) {
        match name {
            Register8::A => self.a = value,
            Register8::B => self.b = value,
            Register8::C => self.c = value,
            Register8::D => self.d = value,
            Register8::E => self.e = value,
            Register8::F => self.f = FlagsRegister::from(value),
            Register8::H => self.h = value,
            Register8::L => self.l = value
        }
    }

    pub fn set_16(&mut self,name:Register16, value:u16) {
        match name {
            Register16::AF => self.set_af(value),
            Register16::BC => self.set_bc(value),
            Register16::DE => self.set_de(value),
            Register16::HL => self.set_hl(value),
            Register16::SP => self.sp = value,
            Register16::PC => self.pc = value,
        }
    }

    pub fn get_16(&self,name:Register16) -> u16 {
        match name {
            Register16::BC => self.get_bc(),
            Register16::DE => self.get_de(),
            Register16::HL => self.get_hl(),
            Register16::SP => self.sp,
            Register16::PC => self.pc,
            Register16::AF => self.get_af()
        }
    }

    fn get_af(&self) -> u16 {
        (self.a as u16) << 8
        | u8::from(&self.f) as u16
    }

    fn set_af(&mut self, value:u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = ((value & 0xFF) as u8).into()
    }

    fn get_bc(&self) -> u16 {
        (self.b as u16) << 8
        | self.c as u16
    }

    fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8
    }

    fn get_de(&self) -> u16 {
        (self.d as u16) << 8
        | self.e as u16
    }

    fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8
    }

    fn get_hl(&self) -> u16 {
        (self.h as u16) << 8
        | self.l as u16
    }

    fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn flag_from_u8() {
        let flags: FlagsRegister = 0xB0.into();
        assert!(flags.zero && !flags.subtract && flags.half_carry && flags.carry);
    }
    #[test]
    fn u8_from_flag() {
        let flags: FlagsRegister = 0xB0.into();
        let flagsu8 = u8::from(&flags);
        assert_eq!(flagsu8,0xB0);
    }
    #[test]
    fn get_u8() {
        let register = Registers::new();
        assert_eq!(register.get_8(Register8::A),register.a);
        assert_eq!(register.get_8(Register8::B),register.b);
        assert_eq!(register.get_8(Register8::C),register.c);
        assert_eq!(register.get_8(Register8::D),register.d);
        assert_eq!(register.get_8(Register8::E),register.e);
        assert_eq!(register.get_8(Register8::H),register.h);
        assert_eq!(register.get_8(Register8::L),register.l);
    }
    #[test]
    fn get_u16() {
        let register = Registers::new();
        assert_eq!(register.get_16(Register16::AF),register.get_af());
        assert_eq!(register.get_16(Register16::BC),register.get_bc());
        assert_eq!(register.get_16(Register16::DE),register.get_de());
        assert_eq!(register.get_16(Register16::HL),register.get_hl());
    }
}