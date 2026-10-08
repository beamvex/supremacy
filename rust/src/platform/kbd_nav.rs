/// Menu cursor navigation direction — which `+0x10`-relative field of a
/// 20-byte hotspot record the scancode selects (`0x2A20E` onwards).
///
/// Arrows `0x48`/`0x50`/`0x4B`/`0x4D` map to up/down/left/right; Tandy
/// mode (`[0x9CC6] == 1`) additionally accepts the Tandy 1000's extra
/// cursor keys `0x29`/`0x4A`/`0x2B`/`0x4E`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavDir {
    /// Field `+0x10` — next item upward.
    Up,
    /// Field `+0x11` — downward.
    Down,
    /// Field `+0x12` — left.
    Left,
    /// Field `+0x13` — right.
    Right,
}

/// Scancode → [`NavDir`], `tga` widens the table like the asm does.
#[must_use]
pub fn nav_dir(scan: u8, tga: bool) -> Option<NavDir> {
    Some(match scan {
        0x48 => NavDir::Up,
        0x50 => NavDir::Down,
        0x4B => NavDir::Left,
        0x4D => NavDir::Right,
        0x29 if tga => NavDir::Up,
        0x4A if tga => NavDir::Down,
        0x2B if tga => NavDir::Left,
        0x4E if tga => NavDir::Right,
        _ => return None,
    })
}
