use crate::gph::{fix_up, parse_index, Record};
use std::fs;
use std::io;
use std::path::Path;

/// Bytes of index the game reads per set — `mov cx,0x10B0` at `0x320AC`
/// (room for 356 records; smaller files simply read short).
const INDEX_LEN: usize = 0x10B0;

/// Port of the `.GPH` loader at `0x3208E`-`0x320E9`: read the index into
/// `ds:0x158`, then run the `0x320D8` fixup loop adding `heap_seg` to every
/// record's `ofs_para`.
///
/// # Errors
/// Propagates any `fs::read` failure (the asm retries via the disk-swap
/// prompt at `0x8AC0` instead).
pub fn load_index(path: &Path, heap_seg: u16) -> io::Result<Vec<Record>> {
    let mut buf = fs::read(path)?;
    buf.truncate(INDEX_LEN);
    let mut records = parse_index(&buf);
    fix_up(&mut records, heap_seg);
    Ok(records)
}
