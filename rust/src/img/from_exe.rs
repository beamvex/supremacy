//! The same file layout [`image`][super::image] rebuilds, produced
//! straight from a packed `GAME.EXE` — for callers (difftest's port
//! side) that must run the bytes of a specific exe on disk rather than
//! the committed blobs.

use super::{IMAGE_LEN, MZ_HDR};

/// EXEPACK-unpack `file` and lay the load image down after the zeroed
/// [`MZ_HDR`] stub the reference carries — the result is
/// interchangeable with a `std::fs::read` of `GAME.unpacked.exe` for
/// every byte the port reads.
///
/// `None` when `file` isn't a valid EXEPACK executable. A load image
/// sized differently from the reference's is truncated or zero-padded
/// to [`IMAGE_LEN`] so indexed reads (`State::from_image` &co.) never
/// see a short buffer.
#[must_use]
pub fn from_exe(file: &[u8]) -> Option<Vec<u8>> {
    let u = crate::exepack::unpack(file).ok()?;
    let mut img = vec![0; IMAGE_LEN];
    let n = u.image.len().min(IMAGE_LEN - MZ_HDR);
    img[MZ_HDR..MZ_HDR + n].copy_from_slice(&u.image[..n]);
    Some(img)
}
