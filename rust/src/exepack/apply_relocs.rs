use super::Error;

/// Apply the segment fixups — `add [es:di],bx` at `0x23B35` where `bx` is
/// the load segment. A `.exe` file stores only the table (see
/// [`crate::exepack::Unpacked::to_exe`]); call this to relocate the image
/// for a given base, as the stub does before the far jump.
///
/// # Errors
/// `Error::Corrupt` if a fixup site is outside the image.
pub fn apply_relocs(image: &mut [u8], relocs: &[u32], base: u16) -> Result<(), Error> {
    for &r in relocs {
        let i = usize::try_from(r).map_err(|_| Error::Corrupt(0))?;
        let b = image.get_mut(i..i + 2).ok_or(Error::Corrupt(0))?;
        let v = u16::from_le_bytes([b[0], b[1]]).wrapping_add(base);
        b.copy_from_slice(&v.to_le_bytes());
    }
    Ok(())
}
