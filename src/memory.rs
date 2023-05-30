use crate::utils;
use selecting::Selector;
use std::result::Result;

const MEMORY_MAX: usize = 1 << 16;

static mut MEMORY: [u16; MEMORY_MAX] = [0; MEMORY_MAX];

static mut COUNTER: usize = 0;

const MR_KBSR: usize = 0xfe00;
const MR_KBDR: usize = 0xfe02;

#[derive(Debug)]
pub enum MemoryError {
    MemoryOverflow,
}

pub fn read(loc: u16) -> u16 {
    unsafe {
        if loc as usize == MR_KBSR {
            let ptr = MEMORY.as_mut_ptr();
            if check_key() {
                *ptr.add(MR_KBSR) = 1 << 15;
                *ptr.add(MR_KBDR) = utils::get_char() as u16;
            } else {
                *ptr.add(MR_KBSR) = 0;
            }
        }

        let ptr = MEMORY.as_ptr();
        *ptr.add(loc as usize)
    }
}

pub fn write(loc: u16, v: u16) -> Result<(), MemoryError> {
    if loc as usize >= MEMORY_MAX {
        return Err(MemoryError::MemoryOverflow);
    }

    unsafe {
        let ptr = MEMORY.as_mut_ptr();
        *ptr.add(loc as usize) = v;
        COUNTER += 1;
    }

    Ok(())
}

fn check_key() -> bool {
    let mut selector = Selector::new();
    selector.add_read(&std::io::stdin());
    let res = selector.try_select().expect("try select error");
    res.is_read(&std::io::stdin())
}
