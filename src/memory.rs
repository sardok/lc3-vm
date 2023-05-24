const MEMORY_MAX: usize = 1 << 16;

const MEMORY: [u16; MEMORY_MAX] = [0; MEMORY_MAX];

pub(super) fn read(loc: u16) -> u16 {
    0
}

pub(super) fn write(loc: u16, v: u16) {
    ()
}
