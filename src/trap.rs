pub(super) enum Trap {
    GETC = 0x20,
    OUT = 0x21,
    PUTS = 0x22,
    IN = 0x23,
    PUTSP = 0x24,
    HALT = 0x25,
}

impl TryFrom<u16> for Trap {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            x if x == Trap::GETC as u16 => Ok(Trap::GETC),
            x if x == Trap::OUT as u16 => Ok(Trap::OUT),
            x if x == Trap::PUTS as u16 => Ok(Trap::PUTS),
            x if x == Trap::IN as u16 => Ok(Trap::IN),
            x if x == Trap::PUTSP as u16 => Ok(Trap::PUTSP),
            x if x == Trap::HALT as u16 => Ok(Trap::HALT),
            _ => Err(()),
        }
    }
}
