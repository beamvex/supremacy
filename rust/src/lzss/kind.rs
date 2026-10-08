/// Draw mode — `mode_flag` from the `.GPH` record (`dl` at `0x2FB80`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// Opaque blit — `stosb` (`0x2EBEF`/`0x2FC5D`).
    Opaque,
    /// XOR overlay — `xor [es:di],al` (`0x2EC8C`/`0x2FCF9`).
    Xor,
}
