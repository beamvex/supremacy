/// Video mode — the index stored at `[0x9CC6]` (`ah` at `0x3202D`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Video {
    /// CGA 320×200×4 — parameter template at dgroup `0xD3A2`, index 0.
    Cga,
    /// Tandy 320×200×16 — template `0xE4FD`, index 1.
    Tga,
    /// EGA 320×200×16 planar — template `0xC247`, index 2.
    Ega,
    /// MCGA/VGA 320×200×256 — template `0xB0EC`, index 3 (default).
    Mcga,
}
