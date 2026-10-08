/// One EXEPACK command block, decoded (`0x23AF4`-`0x23B14`).
pub(super) enum Cmd {
    /// `0xB0` — fill `len` bytes with `byte` (`lodsb` + `rep stosb`).
    Fill {
        /// The byte to write.
        byte: u8,
        /// Run length (`cx`).
        len: usize,
        /// `cmd & 1` — last block, stop after this one.
        last: bool,
    },
    /// `0xB2` — copy `len` bytes ending at `src_end` (`rep movsb`, `DF=1`).
    Copy {
        /// Highest-addressed source byte; the copy reads downwards.
        src_end: usize,
        /// Byte count (`cx`).
        len: usize,
        /// `cmd & 1` — last block.
        last: bool,
    },
}
