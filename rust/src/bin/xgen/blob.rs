//! `data/*.bin` access — `u8`/`u16` reads in each blob's own address
//! space (the `ds:`/`cs:`/module offsets the docs use).

/// The three committed regions, exposed by *their* offset space:
/// `ds` = `dgroup.bin` (`ds:` offset), `cs` = `cs.bin` (`cs:` near
/// offset), `low` = `low.bin` (driver module offset, file − `0xC00`).
pub struct Blobs {
    /// `dgroup.bin` — `ds:0..`
    pub ds: Vec<u8>,
    /// `cs.bin` — `cs:0..`
    pub cs: Vec<u8>,
    /// `low.bin` — file `0xC00..` (PC module `0x0`, Tandy `0xD50`, A/R
    /// `0x1D00`, embedded songs).
    pub low: Vec<u8>,
}

impl Blobs {
    /// Read the three blobs from `dir`.
    pub fn load(dir: &std::path::Path) -> Self {
        let rd = |n: &str| std::fs::read(dir.join(n)).unwrap_or_else(|e| panic!("{n}: {e}"));
        Self {
            ds: rd("dgroup.bin"),
            cs: rd("cs.bin"),
            low: rd("low.bin"),
        }
    }
}

/// `u8` at `ofs`, `0` past the end.
pub fn b8(b: &[u8], ofs: u16) -> u8 {
    b.get(usize::from(ofs)).copied().unwrap_or(0)
}

/// Little-endian `u16` at `ofs`.
pub fn b16(b: &[u8], ofs: u16) -> u16 {
    u16::from(b8(b, ofs)) | u16::from(b8(b, ofs.wrapping_add(1))) << 8
}
