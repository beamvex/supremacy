/// Failure modes of [`crate::exepack::unpack`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// Not an `MZ`/`ZM` executable, or the header is truncated.
    BadMz,
    /// No `RB` EXEPACK header at the entry `CS` paragraph — the file is not
    /// EXEPACK-packed (the stub prints "Packed file is corrupt" here too).
    MissingPack,
    /// Unknown command byte or a command ran off either end of the image
    /// (`0x23B79` prints the corrupt-file message).
    Corrupt(u8),
}
