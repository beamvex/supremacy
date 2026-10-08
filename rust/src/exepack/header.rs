/// The 16-byte EXEPACK header at the start of the packed block
/// (`GAME.EXE:0x23A80`).
#[derive(Debug, Clone, Copy)]
pub struct Header {
    /// Real `IP` — far-jmp target `[bx]`/`[bx+1]` after unpacking.
    pub real_ip: u16,
    /// Real `CS` — relocated by `+ dest_base` at `0x23B62`.
    pub real_cs: u16,
    /// Block size in bytes (header + stub + packed fixups) — `cx` at `0x23AA0`.
    pub pack_size: u16,
    /// Real `SP` — `[0x8]` loaded at `0x23B58`.
    pub real_sp: u16,
    /// Real `SS` — `[0xA]` + `dest_base`, loaded at `0x23B5C`.
    pub real_ss: u16,
    /// Destination size in paragraphs — `cx`-equivalent `dest_len`, `[0xC]`.
    pub dest_len: u16,
}
