use crate::memory;
use std::convert::Into;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::result::Result;

enum ReadImageError {
    FsError(std::io::Error),
    MemoryError(memory::MemoryError),
}

impl Into<ReadImageError> for std::io::Error {
    fn into(self: std::io::Error) -> Self {
        ReadImageError::FsError(err)
    }
}

fn read_image_file(file_path: &str) -> Result<(), ReadImageError> {
    let mut file = File::open(file_path)?;
    let mut addr = read_u16(&mut file).map_err(|err| err.into())?;
    loop {
        let res = read_u16(&mut file);
        match res {
            Ok(v) => {
                memory::write(addr, v).map_err(|err| ReadImageError::FsError(err))?;
                addr += 1;
            }
            Err(_) => return Ok(()),
        }
    }
    Ok(())
}

fn read_u16(file: &mut File) -> std::io::Result<u16> {
    let mut buf = [0u8; 2];
    let n = file.read(&mut buf)?;
    if n < 2 {
        return Err(std::io::Error::new(
            io::ErrorKind::InvalidInput,
            "insufficient data",
        ));
    }
    assert_eq!(n, 2);
    Ok(u16::from_be_bytes(buf))
}
