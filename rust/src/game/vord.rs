//! `0xF9` — the orders popup (`cs:0x4309`, file `0x34309`–`0x3442D`).
//! Gated on `[0x91F4]` and UI mode `7`; frames a window, prints the
//! header strings, picks a `0x1E`-byte entry from `ds:0x4EBA` by the
//! selected record's fields, loads its block into the message cells and
//! refreshes.

use super::consts::SEL_REC;
use super::text;
use super::vhost::Call;
use super::vops::{uimode, Ctx};

/// `ds:` gate byte — nonzero means an order is pending (cleared at the
/// end, file `0x34309`/`0x34428`).
const ORD_FLAG: u16 = 0x91F4;
/// `ds:` popup-active word — nonzero suppresses a second open
/// (file `0x3431A`).
const ORD_OPEN: u16 = 0x9198;
/// `ds:` cycling selector `0..4` for the `+0x26 != 0` case — `+1`, wraps
/// `5 → 2` (file `0x34355`–`0x34366`).
const ORD_CYC: u16 = 0x91AC;
/// `ds:` entry table — `0x1E`-byte order/message blocks (file
/// `0x343CB`).
const ORD_TAB: u16 = 0x4EBA;

/// Record field `+0x26` — branch A of the pick (file `0x34349`).
const F_26: u16 = 0x26;
/// Record field `+0x2E` — the order-pending test (file `0x34342`).
const F_2E: u16 = 0x2E;
/// Record field `+0x0C` — the kind test (file `0x34374`).
const F_KIND: u16 = 0x0C;

/// `cs:0x4309` — run one orders-popup pass. Returns `true` when the
/// window actually opened (the asm falls through to the draw block).
pub fn orders(c: &mut Ctx) -> bool {
    if c.vm.r8(c.st, ORD_FLAG) == 0 || uimode(c) != 7 || c.vm.r16(c.st, ORD_OPEN) != 0 {
        return false;
    }
    c.host.svc(Call::Rect(0x1C, 0x6, 0x3, 0x41));
    text::print_str(c, 0x4F50, 0x38, 0x1E);
    let Some(bx) = pick(c) else {
        text::print_str(c, 0x4F6E, 0x38, 0x1E);
        return true;
    };
    text::print_str(c, 0x4F8D, 0x38, 0x1E);
    text::print_str(c, 0x4FA6, 0x39, 0x39);
    load_entry(c, ORD_TAB + bx * 0x1E);
    c.vm.w8(c.st, ORD_FLAG, 0);
    true
}

/// The `bx` selection (file `0x3433E`–`0x34397`): `+0x2E` pending picks
/// `1` or the cycled value, else `+0x26`/kind pick `0`/`1`; any other
/// kind prints the fallback line and returns `None`.
fn pick(c: &mut Ctx) -> Option<u16> {
    let si = c.vm.r16(c.st, u16::try_from(SEL_REC).unwrap_or(0));
    if c.vm.r16(c.st, si + F_2E) != 0 {
        return Some(if c.vm.r16(c.st, si + F_26) == 0 {
            1
        } else {
            cycle(c)
        });
    }
    if c.vm.r16(c.st, si + F_26) != 0 {
        return Some(0);
    }
    match c.vm.r16(c.st, si + F_KIND) {
        0xA => Some(1),
        7 => Some(0),
        _ => None,
    }
}

/// `bx = [0x91AC] + 1`, wrapped `5 → 2` (file `0x34355`–`0x34366`).
fn cycle(c: &mut Ctx) -> u16 {
    let mut bx = c.vm.r16(c.st, ORD_CYC).wrapping_add(1);
    if bx == 5 {
        bx = 2;
    }
    c.vm.w16(c.st, ORD_CYC, bx);
    bx
}

/// Entry `si`: `call 0x2CA9`, clear `[0x91A0]`/`[0x91D7]`, then
/// `lodsw` copies — word `→ [0x9198]`, 12 words `→ [0x9255..0x926C]`,
/// one byte `→ [0x91D6]` — refresh and the `0x5C49` tail call (file
/// `0x343CB`–`0x34428`).
fn load_entry(c: &mut Ctx, si: u16) {
    c.host.svc(Call::Ui(si));
    c.vm.w16(c.st, 0x91A0, 0);
    c.vm.w8(c.st, 0x91D7, 0);
    c.vm.w16(c.st, ORD_OPEN, c.vm.r16(c.st, si));
    for k in 0..12 {
        c.vm.w16(c.st, 0x9255 + k * 2, c.vm.r16(c.st, si + 2 + k * 2));
    }
    c.vm.w8(c.st, 0x91D6, c.vm.r8(c.st, si + 0x1A));
    c.host.svc(Call::Refresh);
    c.host.svc(Call::Native(0x5C49));
}
