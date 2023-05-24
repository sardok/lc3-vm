use std::io;
use std::io::prelude::*;

use crate::{
    opcode::{ConditionFlag, Opcode},
    register::RegisterBlock,
    trap::Trap,
    utils::sign_extend,
};

mod memory;
mod opcode;
mod register;
mod trap;
mod utils;

use register::Register;
use utils::{get_dr, get_imm5, get_imm_mode, get_sr1, get_sr2};

const PC_START: u16 = 0x3000;

fn main() {
    let mut registers = RegisterBlock::new();
    registers[Register::COND] = opcode::ConditionFlag::ZRO.into();
    registers[Register::PC] = PC_START;

    let mut running = true;
    while running {
        let instr = memory::read(registers.read_and_inc_pc());
        match Opcode::try_from_instruction(instr) {
            Ok(op) => match op {
                Opcode::BR => {
                    let pc_offset = sign_extend(instr & 0x1ff, 9);
                    let n_flag = (instr >> 11) & 0x1 == 1;
                    let z_flag = (instr >> 10) & 0x1 == 1;
                    let p_flag = (instr >> 9) & 0x1 == 1;
                    let cond = registers.read_cond();
                    if (n_flag && cond == ConditionFlag::NEG)
                        || (z_flag && cond == ConditionFlag::ZRO)
                        || (p_flag && cond == ConditionFlag::POS)
                    {
                        registers[Register::PC] += pc_offset;
                    }
                }
                Opcode::ADD => {
                    let dr = get_dr(instr);
                    let sr1 = get_sr1(instr);
                    let imm_mode = get_imm_mode(instr);
                    let v = if imm_mode {
                        let imm5 = get_imm5(instr);
                        registers[sr1].checked_add(imm5).unwrap()
                    } else {
                        let sr2 = get_sr2(instr);
                        registers[sr1].checked_add(registers[sr2]).unwrap()
                    };
                    registers[dr] = v;
                    registers.update_flags(dr);
                }
                Opcode::LD => {
                    let dr = get_dr(instr);
                    let pc_offset = sign_extend(instr & 0x1ff, 9);
                    registers[dr] = memory::read(registers[Register::PC] + pc_offset);
                    registers.update_flags(dr);
                }
                Opcode::ST => {
                    let dr = get_dr(instr);
                    let pc_offset = sign_extend(instr & 0x1ff, 9);
                    memory::write(registers[Register::PC] + pc_offset, registers[dr]);
                }
                Opcode::JSR => {
                    let long_flag = (instr >> 1) & 0x1 == 1;
                    registers[Register::R7] = registers[Register::PC];
                    if long_flag {
                        let long_pc_offset = sign_extend(instr & 0x7ff, 11);
                        let pc = registers[Register::PC].checked_add(long_pc_offset).unwrap();
                        registers[Register::PC] = pc;
                    } else {
                        let sr1 = get_sr1(instr);
                        registers[Register::PC] = registers[sr1]; /* JSRR */
                    }
                }
                Opcode::AND => {
                    let dr = get_dr(instr);
                    let sr1 = get_sr1(instr);
                    let imm_mode = get_imm_mode(instr);
                    let v = if imm_mode {
                        let imm5 = get_imm5(instr);
                        registers[sr1] & imm5
                    } else {
                        let sr2 = get_sr2(instr);
                        registers[sr1] & registers[sr2]
                    };
                    registers.write_and_update(dr, v);
                }
                Opcode::LDR => {
                    let dr = get_dr(instr);
                    let sr1 = get_sr1(instr);
                    let offset = sign_extend(instr & 0x3f, 6);
                    let v = memory::read(registers[sr1] + offset);
                    registers.write_and_update(dr, v);
                }
                Opcode::STR => {
                    let dr = get_dr(instr);
                    let sr1 = get_sr1(instr);
                    let offset = sign_extend(instr & 0x3f, 6);
                    memory::write(registers[sr1] + offset, registers[dr]);
                }
                Opcode::NOT => {
                    let dr = get_dr(instr);
                    let sr1 = get_sr1(instr);
                    registers.write_and_update(dr, !registers[sr1]);
                }
                Opcode::LDI => {
                    let dr = get_dr(instr);
                    let pc_offset = utils::sign_extend(instr & 0x1ff, 9);
                    let v = memory::read(memory::read(registers[Register::PC] + pc_offset));
                    registers[dr] = v;
                    registers.update_flags(dr);
                }
                Opcode::STI => {
                    let dr = get_dr(instr);
                    let pc_offset = sign_extend(instr & 0x1ff, 9);
                    let loc = memory::read(registers[Register::PC] + pc_offset);
                    memory::write(loc, registers[dr]);
                }
                Opcode::JMP => {
                    let sr1 = get_sr1(instr);
                    registers[Register::PC] = registers[sr1];
                }
                Opcode::LEA => {
                    let dr = get_dr(instr);
                    let pc_offset = sign_extend(instr & 0x1ff, 9);
                    let v = registers[Register::PC] + pc_offset;
                    registers.write_and_update(dr, v);
                }
                Opcode::TRAP => {
                    registers[Register::R7] = registers[Register::PC];
                    let trap = Trap::try_from(instr & 0xff).unwrap();
                    match trap {
                        Trap::GETC => {
                            let chr = utils::get_char_and_flush();
                            registers.write_and_update(Register::R0, chr as u16);
                        }
                        Trap::OUT => {
                            let chr = (registers[Register::R0] & 0xFF) as u8;
                            utils::put_char_and_flush(chr);
                        }
                        Trap::PUTS => {
                            let mut loc = registers[Register::R0];
                            loop {
                                let c = memory::read(loc);
                                if c == 0 {
                                    break;
                                }
                                print!("{}", c);
                                loc += 1;
                            }
                            io::stdout().flush().unwrap();
                        }
                        Trap::IN => {
                            let chr = utils::get_char_and_flush();
                            utils::put_char_and_flush(chr);
                            registers.write_and_update(Register::R0, chr as u16);
                        }
                        Trap::PUTSP => {
                            let mut loc = registers[Register::R0];
                            loop {
                                let c = memory::read(loc);
                                if c == 0 {
                                    break;
                                }
                                let c1 = (c & 0xFF) as u8;
                                utils::put_char(c1);
                                let c2 = (c >> 8) as u8;
                                if c2 > 0 {
                                    utils::put_char(c2);
                                }
                                loc += 1;
                            }
                            io::stdout().flush().unwrap();
                        }
                        Trap::HALT => {
                            utils::put_str_and_flush("HALT");
                            running = false;
                        }
                    }
                }
                _ => {
                    unreachable!("Unimplemented opcode!");
                }
            },
            Err(err) => {
                println!("{}", err);
                break;
            }
        }
    }

    println!("Hello, world!");
}
