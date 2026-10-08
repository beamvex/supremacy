use super::mz::Mz;
use super::Error;

impl Mz {
    /// Signature + header size `[0x08]` + image length from the page count
    /// (`[0x02]` bytes-in-last-page, `[0x04]` total pages) + entry `CS`
    /// `[0x16]`.
    pub(crate) fn parse(f: &[u8]) -> Result<Self, Error> {
        let sig = f.get(..2).ok_or(Error::BadMz)?;
        if sig != b"MZ" && sig != b"ZM" {
            return Err(Error::BadMz);
        }
        let w = |o: usize| -> Result<u16, Error> {
            let b = f.get(o..o + 2).ok_or(Error::BadMz)?;
            Ok(u16::from_le_bytes([b[0], b[1]]))
        };
        let last = usize::from(w(0x02)?);
        let pages = usize::from(w(0x04)?);
        let hdr = usize::from(w(0x08)?) * 16;
        let img_len = pages * 512 - if last == 0 { 0 } else { 512 - last } - hdr;
        Ok(Self {
            image: hdr..hdr + img_len,
            entry_cs: w(0x16)?,
        })
    }
}
