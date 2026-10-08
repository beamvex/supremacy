use super::Error;

/// `rep stosb` with `DF=1` (`0x23B04`-`0x23B05`): write `byte` at `di`,
/// `di-1`, … `di-len+1`, leaving `di` past the written run.
pub(super) fn fill(buf: &mut [u8], di: &mut usize, byte: u8, len: usize) -> Result<(), Error> {
    if *di >= buf.len() || *di + 1 < len {
        return Err(Error::Corrupt(0xB0));
    }
    for _ in 0..len {
        buf[*di] = byte;
        *di -= 1;
    }
    Ok(())
}
