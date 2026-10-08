use super::{Record, RECORD_LEN};

/// Parse a `.GPH` index: six little-endian `u16`s per record, any trailing
/// partial record ignored (the game reads a fixed `0x10B0` bytes — up to 356
/// records — into `ds:0x158`).
#[must_use]
pub fn parse_index(buf: &[u8]) -> Vec<Record> {
    buf.as_chunks::<RECORD_LEN>()
        .0
        .iter()
        .map(|c| Record {
            x: u16::from_le_bytes([c[0], c[1]]),
            y: u16::from_le_bytes([c[2], c[3]]),
            w: u16::from_le_bytes([c[4], c[5]]),
            h: u16::from_le_bytes([c[6], c[7]]),
            ofs_para: u16::from_le_bytes([c[8], c[9]]),
            mode_flag: u16::from_le_bytes([c[10], c[11]]),
        })
        .collect()
}
