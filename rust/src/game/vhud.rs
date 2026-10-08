//! The HUD refresh arms — `0xA7` (`cs:0x60A8`), `0xFD` (`cs:0x60B6`),
//! `0xFC` (`cs:0x60D4`) reprint the selected-object and faction stat
//! fields in UI mode 1, and `0xAA` (`cs:0x3738`) marks every sprite slot
//! pending for the redraw pass.

use super::consts::LINK_REC;
use super::num;
use super::rec::{F_CREDITS, F_POP};
use super::vops::{faction_rec, uimode, Ctx};

/// `ds:` base of the sprite pending-flag bytes — `0x80` = dirty, the
/// table is `0xFF`-terminated (file `0x33738`–`0x3374A`; the redraw pass
/// `cs:0x80F0` consumes the same cells).
const SPRITES: u16 = 0x7562;

/// `0xA7` — reprint the selected object's word `+0` at `(0x2C,0xB)`
/// (file `0x360A8`–`0x360B5`; `si = [0x914C]` is the mode-1 selection).
pub fn hud_ship(c: &mut Ctx) {
    if uimode(c) != 1 {
        return;
    }
    let si = c.vm.r16(c.st, u16::try_from(LINK_REC).unwrap_or(0));
    num::num_pad(c, c.vm.r16(c.st, si), 0x2C, 0x0B);
}

/// `0xFD` — reprint the faction record's credits dword at `(0x45,0x32)`
/// (file `0x360B6`–`0x360D3`; `ax:bp = [si+0x36]` → `call 0x68E0`).
pub fn hud_credits(c: &mut Ctx) {
    if uimode(c) != 1 {
        return;
    }
    let si = faction_rec(c) + u16::try_from(F_CREDITS).unwrap_or(0);
    let v = u32::from(c.vm.r16(c.st, si)) | u32::from(c.vm.r16(c.st, si + 2)) << 16;
    num::num32(c, v, 0x45, 0x32);
}

/// `0xFC` — reprint the faction record's population word at `(0x49,0xB)`
/// (file `0x360D4`–`0x360EC`; `ax = [si+0x2A]` → `jmp 0x686C`).
pub fn hud_pop(c: &mut Ctx) {
    if uimode(c) != 1 {
        return;
    }
    let si = faction_rec(c) + u16::try_from(F_POP).unwrap_or(0);
    num::num_pad(c, c.vm.r16(c.st, si), 0x49, 0x0B);
}

/// `0xAA` — set the `0x80` pending bit on every sprite slot up to the
/// `0xFF` terminator (file `0x33738`–`0x3374A`).
pub fn mark_sprites(c: &mut Ctx) {
    let mut si = SPRITES;
    while c.vm.r8(c.st, si) != 0xFF {
        c.vm.w8(c.st, si, c.vm.r8(c.st, si) | 0x80);
        si = si.wrapping_add(1);
    }
}
