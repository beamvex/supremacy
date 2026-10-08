//! The `0x20..0x38` dispatcher arm — per-ship crew ageing plus the
//! selected-ship status refresh (`cs:0x60EE`, file `0x360EE`–`0x361E4`).

use super::consts::{DIFFICULTY, FLEET_BASE, FLEET_STRIDE, LINK_REC};
use super::ship::{S_CREW, S_FLAGS, S_POW};
use super::vhost::Call;
use super::vops::{uimode, Ctx};

/// `ds:` slot holding the ship pointer while its panel updates
/// (`[0x9150]` — shared scratch, the defence pass uses it too).
const SHIP_PTR: u16 = 0x9150;
/// `ds:` cell the rating display byte is written to — `9 − crew/16`,
/// or `0xFF` while the ship is uncrewed (file `0x3615B`–`0x3617E`).
const SHIP_RATE: u16 = 0x91D6;
/// `ds:` cell for the "active crew" flag — `0xFF` while the crew byte
/// is nonzero (file `0x3617F`–`0x36191`).
const SHIP_ACTIVE: u16 = 0x91DE;
/// `ds:` base of the rating-line table — `0x13`-byte entries indexed by
/// `crew/10` (file `0x361BF`–`0x361E4`).
const RATE_TAB: u16 = 0x7438;
/// Crew clamp (file `0x3612E`).
const CREW_MAX: u8 = 0x64;

/// What a `ship_tick` did — the asm `ret`s unless the record is the
/// UI-mode-1 selected ship.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ShipOut {
    /// `[si+6] & 2` — record skipped before ageing.
    Skip,
    /// Crew advanced; not the selected ship (or UI mode `!= 1`).
    Aged,
    /// Crew advanced and the status cells/panel refreshed.
    Panel,
}

/// One ship's tick — `bx = code − 0x20` indexes the `ds:0x9991` array.
/// The crew byte gains `3`/`2`/`1` by difficulty `0`/`1`/`2`, clamped
/// to `0x64`; the UI-mode-1 selected ship additionally refreshes
/// `[0x91D6]`/`[0x91DE]` and prints crew + rating line.
pub fn ship_tick(c: &mut Ctx, i: u16) -> ShipOut {
    let si = u16::try_from(FLEET_BASE).unwrap_or(0) + i * u16::try_from(FLEET_STRIDE).unwrap_or(0);
    if c.vm.r8(c.st, si + flag()) & 2 != 0 {
        return ShipOut::Skip;
    }
    age(c, si);
    if uimode(c) != 1 || c.vm.r16(c.st, u16::try_from(LINK_REC).unwrap_or(0)) != si {
        return ShipOut::Aged;
    }
    panel(c, si);
    ShipOut::Panel
}

/// `inc byte [si+7]` once, again for `diff != 2`, again for `diff ==
/// 0`; clamp `0x64` (file `0x36117`–`0x36137`).
fn age(c: &mut Ctx, si: u16) {
    let d = c.vm.r8(c.st, u16::try_from(DIFFICULTY).unwrap_or(0));
    let mut t = c.vm.r8(c.st, si + crew()).wrapping_add(1);
    if d != 2 {
        t = t.wrapping_add(1);
        if d == 0 {
            t = t.wrapping_add(1);
        }
    }
    c.vm.w8(c.st, si + crew(), t.min(CREW_MAX));
}

/// The `cs:0x6142`–`0x61E4` UI half: stash the pointer, zero the crew
/// when `word[si] == 0`, derive rating + active flag, print crew at
/// `(0x46,0xC0)` and the `crew/10`-indexed status line at `(0x3B,0x3F)`.
fn panel(c: &mut Ctx, si: u16) {
    c.vm.w16(c.st, SHIP_PTR, si);
    if c.vm.r16(c.st, si + pow()) == 0 {
        c.vm.w8(c.st, si + crew(), 0);
    }
    let t = c.vm.r8(c.st, si + crew());
    let rate = if t == 0 { 0xFF } else { 9 - (t >> 4) };
    c.vm.w8(c.st, SHIP_RATE, rate);
    c.vm.w8(c.st, SHIP_ACTIVE, if t == 0 { 0 } else { 0xFF });
    c.host.svc(Call::Num(u32::from(t), 0x46, 0xC0));
    let row = RATE_TAB + u16::from(t / 0xA) * 0x13;
    c.host.svc(Call::Text(row, 0x3B, 0x3F));
}

fn flag() -> u16 {
    u16::try_from(S_FLAGS).unwrap_or(0)
}

fn crew() -> u16 {
    u16::try_from(S_CREW).unwrap_or(0)
}

fn pow() -> u16 {
    u16::try_from(S_POW).unwrap_or(0)
}
