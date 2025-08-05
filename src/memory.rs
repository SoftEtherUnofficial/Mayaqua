// memory.rs - Exact translation of memory.go

use std::io::{self, Read, Write};

// ReadBufStr read string from buffer
pub fn read_buf_str<R: Read>(r: &mut R) -> io::Result<String> {
    let mut num_bytes = [0u8; 4];
    r.read_exact(&mut num_bytes)?;
    let num = u32::from_be_bytes(num_bytes);
    
    if num == 0 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid string"));
    }
    
    let num = num - 1;
    let mut buf = vec![0u8; num as usize];
    r.read_exact(&mut buf)?;
    
    String::from_utf8(buf)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8"))
}

// WriteBufStr write string to buffer
pub fn write_buf_str<W: Write>(w: &mut W, s: &str) -> io::Result<()> {
    let b = s.as_bytes();
    let num = (b.len() as u32) + 1;
    w.write_all(&num.to_be_bytes())?;
    w.write_all(b)?;
    Ok(())
}
