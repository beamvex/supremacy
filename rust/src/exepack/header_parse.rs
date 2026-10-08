use super::consts::SIG;
use super::{Error, Header};

impl Header {
    /// Parse and signature-check the 16-byte header at `block[0..16]`.
    pub(crate) fn parse(block: &[u8]) -> Result<Self, Error> {
        if block.len() < 16 || block.get(14..16) != Some(&SIG[..]) {
            return Err(Error::MissingPack);
        }
        let w = |o| u16::from_le_bytes([block[o], block[o + 1]]);
        Ok(Self {
            real_ip: w(0x0),
            real_cs: w(0x2),
            pack_size: w(0x6),
            real_sp: w(0x8),
            real_ss: w(0xA),
            dest_len: w(0xC),
        })
    }
}
