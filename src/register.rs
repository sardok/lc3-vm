use std::convert::{From, TryFrom};
use std::ops::{Index, IndexMut};

use crate::opcode::ConditionFlag;

#[derive(Debug, Clone, Copy)]
pub enum Register {
    R0 = 0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    PC,
    COND,
    COUNT,
}

pub struct RegisterBlock([u16; Register::COUNT as usize]);

impl RegisterBlock {
    pub fn new() -> Self {
        Self([0; Register::COUNT as usize])
    }

    pub fn read_and_inc_pc(&mut self) -> u16 {
        let v = self[Register::PC];
        self.0[Register::PC as usize] += 1;
        v
    }

    pub fn write_and_update(&mut self, r: Register, v: u16) {
        self[r] = v;
        self.update_flags(r);
    }

    pub fn update_flags(&mut self, r: Register) {
        let condition_flag = if self[r] == 0 {
            ConditionFlag::ZRO
        } else if self[r] >> 15 > 0 {
            ConditionFlag::NEG
        } else {
            ConditionFlag::POS
        };
        self[Register::COND] = condition_flag as u16;
    }

    pub fn read_cond(&self) -> ConditionFlag {
        let v = self[Register::COND];
        v.into()
    }
}

impl Index<Register> for RegisterBlock {
    type Output = u16;

    fn index(&self, r: Register) -> &Self::Output {
        &self.0[r as usize]
    }
}

impl IndexMut<Register> for RegisterBlock {
    fn index_mut(&mut self, index: Register) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

impl From<Register> for usize {
    #[inline]
    fn from(r: Register) -> usize {
        r as usize
    }
}

impl TryFrom<u16> for Register {
    type Error = ();

    #[inline]
    fn try_from(v: u16) -> Result<Self, Self::Error> {
        match v {
            x if x == Register::R0 as u16 => Ok(Register::R0),
            x if x == Register::R1 as u16 => Ok(Register::R1),
            x if x == Register::R2 as u16 => Ok(Register::R2),
            x if x == Register::R3 as u16 => Ok(Register::R3),
            x if x == Register::R4 as u16 => Ok(Register::R4),
            x if x == Register::R5 as u16 => Ok(Register::R5),
            x if x == Register::R6 as u16 => Ok(Register::R6),
            x if x == Register::R7 as u16 => Ok(Register::R7),
            x if x == Register::PC as u16 => Ok(Register::PC),
            x if x == Register::COND as u16 => Ok(Register::COND),
            x if x == Register::COUNT as u16 => Ok(Register::COUNT),
            _ => Err(()),
        }
    }
}
