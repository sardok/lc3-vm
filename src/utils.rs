use crate::register::Register;
use std::io;
use std::io::prelude::*;

#[inline]
pub fn sign_extend(v: u16, bit_count: u16) -> u16 {
    let mut ret = v;
    if (ret >> (bit_count - 1)) & 1 == 1 {
        ret |= 0xffff << bit_count;
    }
    ret
}

#[inline]
pub fn get_imm_mode(instr: u16) -> bool {
    (instr >> 5) & 0x1 == 1
}

#[inline]
pub fn get_dr(instr: u16) -> Register {
    Register::try_from((instr >> 9) & 0x7).unwrap()
}

#[inline]
pub fn get_sr1(instr: u16) -> Register {
    Register::try_from((instr >> 6) & 0x7).unwrap()
}

#[inline]
pub fn get_sr2(instr: u16) -> Register {
    Register::try_from(instr & 0x7).unwrap()
}

#[inline]
pub fn get_imm5(instr: u16) -> u16 {
    assert_eq!(get_imm_mode(instr), true);
    sign_extend(instr & 0x1f, 5)
}

#[inline]
pub fn get_char_and_flush() -> u8 {
    let mut buffer = [0u8; 1];
    io::stdin().read(&mut buffer[..]).unwrap();
    buffer[0]
}

#[inline]
pub fn put_char(v: u8) {
    io::stdout().write(&[v]).unwrap();
}

#[inline]
pub fn put_char_and_flush(v: u8) {
    io::stdout().write(&[v]).unwrap();
    io::stdout().flush().unwrap();
}

#[inline]
pub fn put_str_and_flush(s: &str) {
    io::stdout().write(s.as_bytes()).unwrap();
    io::stdout().flush().unwrap();
}
