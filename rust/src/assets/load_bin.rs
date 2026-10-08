use std::fs;
use std::io;
use std::path::Path;

/// Port of the `.BIN` loader at `0x320F2`-`0x3215A`: the asm loops
/// `int 21/3F` with `cx=0xFFF0`, bumping `ds` by `0xFFF` paragraphs per
/// chunk until a short read. Flat host memory makes it a single read — the
/// result is identical.
///
/// # Errors
/// Propagates any `fs::read` failure.
pub fn load_bin(path: &Path) -> io::Result<Vec<u8>> {
    fs::read(path)
}
