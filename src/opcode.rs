use std::convert::{From, TryFrom};

pub(super) enum Opcode {
    BR = 0, /* branch */
    ADD,    /* add */
    LD,     /* load */
    ST,     /* store */
    JSR,    /* jump register */
    AND,    /* bitwise and */
    LDR,    /* load register */
    STR,    /* store register */
    RTI,    /* unused */
    NOT,    /* bitwise not */
    LDI,    /* load indirect */
    STI,    /* store indirect */
    JMP,    /* jump */
    RES,    /* reserved (unused) */
    LEA,    /* load effective address */
    TRAP,   /* execute trap */
}

#[derive(PartialEq)]
pub enum ConditionFlag {
    POS = 1 << 0, /* P */
    ZRO = 1 << 1, /* Z */
    NEG = 1 << 2, /* N */
}

impl From<Opcode> for u16 {
    #[inline]
    fn from(o: Opcode) -> u16 {
        o as u16
    }
}

impl From<ConditionFlag> for u16 {
    #[inline]
    fn from(c: ConditionFlag) -> u16 {
        c as u16
    }
}

impl TryFrom<u16> for Opcode {
    type Error = ();

    #[inline]
    fn try_from(v: u16) -> Result<Self, Self::Error> {
        match v {
            x if x == Opcode::BR as u16 => Ok(Opcode::BR),
            x if x == Opcode::ADD as u16 => Ok(Opcode::ADD),
            x if x == Opcode::LD as u16 => Ok(Opcode::LD),
            x if x == Opcode::ST as u16 => Ok(Opcode::ST),
            x if x == Opcode::JSR as u16 => Ok(Opcode::JSR),
            x if x == Opcode::AND as u16 => Ok(Opcode::AND),
            x if x == Opcode::LDR as u16 => Ok(Opcode::LDR),
            x if x == Opcode::STR as u16 => Ok(Opcode::STR),
            x if x == Opcode::RTI as u16 => Ok(Opcode::RTI),
            x if x == Opcode::NOT as u16 => Ok(Opcode::NOT),
            x if x == Opcode::LDI as u16 => Ok(Opcode::LDI),
            x if x == Opcode::STI as u16 => Ok(Opcode::STI),
            x if x == Opcode::JMP as u16 => Ok(Opcode::JMP),
            x if x == Opcode::RES as u16 => Ok(Opcode::RES),
            x if x == Opcode::LEA as u16 => Ok(Opcode::LEA),
            x if x == Opcode::TRAP as u16 => Ok(Opcode::TRAP),
            _ => Err(()),
        }
    }
}

impl From<u16> for ConditionFlag {
    fn from(v: u16) -> Self {
        match v {
            x if x == ConditionFlag::NEG as u16 => ConditionFlag::NEG,
            x if x == ConditionFlag::POS as u16 => ConditionFlag::POS,
            x if x == ConditionFlag::ZRO as u16 => ConditionFlag::ZRO,
            _ => panic!("Invalid condition {}", v),
        }
    }
}

impl Opcode {
    pub fn try_from_instruction(instr: u16) -> Result<Self, String> {
        let op = instr >> 12;
        match op.try_into() {
            Ok(op) => Ok(op),
            Err(_) => Err(format!("Unable to extract opcode from {:#x}", instr)),
        }
    }
}
