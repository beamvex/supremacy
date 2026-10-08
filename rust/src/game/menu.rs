//! The menu service — `cs:0xA10F` (file `0x2A10F`–`0x2A18D`), called
//! once per frame from the main loop. Polled input first (`0xA1CA`
//! keyboard path, `0xA18E` synthetic mouse codes), then a debounced
//! hit-test over the `0x14`-byte hotspot list `[0x9194]`/`[0x91A8]`.
//!
//! Hotspot record: `+0/+2/+4/+6` = `x1,y1,x2,y2` (inclusive), `+8` =
//! pressed-state image index (drawn via `[0x125A]` when nonzero), `+0xC`
//! = action `cs:` pointer (`jmp [si+0xC]`), `+0x10..0x13` = nav links
//! for up/down/left/right (`0xFF` = stay).

use super::consts::{
    ARMED, BUTTONS, CUR_X, CUR_Y, DRAG, HOT_COUNT, HOT_IMG, HOT_LIST, HOT_SEL, KEY_CODE, KEY_PEND,
    K_MODE, TANDY,
};
use super::vhost::Call;
use super::vops::Ctx;

/// `u16` casts for the `ds:` slot constants.
macro_rules! a16 {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// Hotspot record stride (file `0x2A170`).
const HOT_STRIDE: u16 = 0x14;
/// Cursor-centre clamp (`y ≤ 0xBC`, file `0x2A2FD`).
const Y_MAX: u16 = 0xBC;

/// The `cs:0xA10F` body — key service, then the debounced hit-test.
pub fn service(c: &mut Ctx) {
    keys(c);
    if c.vm.r8(c.st, a16!(DRAG)) & 1 != 0 {
        return;
    }
    if c.vm.r16(c.st, a16!(BUTTONS)) & 1 == 0 {
        if c.vm.r8(c.st, a16!(ARMED)) & 1 != 0 {
            c.vm.w8(c.st, a16!(ARMED), 0);
        }
        return;
    }
    if c.vm.r8(c.st, a16!(ARMED)) & 1 != 0 {
        return;
    }
    hit_test(c);
}

/// The hit-test loop (file `0x2A128`–`0x2A176`) — on a hit: draw the
/// pressed image, latch `[0x91CA]`, stash `[0x9CC8]` at `[list − 2]`,
/// dispatch `+0xC`.
fn hit_test(c: &mut Ctx) {
    let (cx, dy) = (c.vm.r16(c.st, a16!(CUR_X)), c.vm.r16(c.st, a16!(CUR_Y)));
    let list = c.vm.r16(c.st, a16!(HOT_LIST));
    let count = c.vm.r16(c.st, a16!(HOT_COUNT));
    for i in 0..count {
        let si = list.wrapping_add(i.wrapping_mul(HOT_STRIDE));
        if inside(c, si, cx, dy) {
            hit(c, si, list);
            return;
        }
    }
}

/// Inclusive bounds test — `x1 ≤ cx ≤ x2`, `y1 ≤ dy ≤ y2`.
fn inside(c: &Ctx, si: u16, cx: u16, dy: u16) -> bool {
    c.vm.r16(c.st, si) <= cx
        && c.vm.r16(c.st, si + 2) <= dy
        && c.vm.r16(c.st, si + 4) >= cx
        && c.vm.r16(c.st, si + 6) >= dy
}

/// The hit body (file `0x2A14B`–`0x2A16C`).
fn hit(c: &mut Ctx, si: u16, list: u16) {
    let img = c.vm.r16(c.st, si + 8);
    c.vm.w16(c.st, a16!(HOT_IMG), img);
    if img != 0 {
        c.host.svc(Call::Image(img));
    }
    c.vm.w8(c.st, a16!(ARMED), 1);
    let sel = c.vm.r16(c.st, a16!(HOT_SEL));
    c.vm.w16(c.st, list.wrapping_sub(2), sel);
    let action = c.vm.r16(c.st, si + 0xC);
    super::actions::dispatch_action(c, action);
}

/// `cs:0xA1CA` — synthetic codes for the mouse path, then the `K`-mode
/// scancode branch (file `0x2A1CA`–`0x2A2D1`). Also the `0xA348`
/// channel-B half of the `0xE66E` line editor (`super::linein`).
pub(crate) fn keys(c: &mut Ctx) {
    synth(c);
    if c.vm.r8(c.st, a16!(K_MODE)) != 1 {
        return;
    }
    if c.vm.r8(c.st, a16!(KEY_PEND)) != 1 {
        snap(c);
        return;
    }
    c.vm.w8(c.st, a16!(KEY_PEND), 0);
    match c.vm.r8(c.st, a16!(KEY_CODE)) {
        0x1C => press(c, 1),
        0x01 => press(c, 2),
        0x81 | 0x9C => press(c, 0),
        code => nav(c, code),
    }
}

/// `cs:0xA18E` — driver-side synthetic codes `0x7E`/`0x7D` (down) and
/// `0xFE`/`0xFD` (release); skipped entirely in `K` mode.
fn synth(c: &mut Ctx) {
    if c.vm.r8(c.st, a16!(K_MODE)) & 1 != 0 {
        return;
    }
    if c.vm.r8(c.st, a16!(KEY_PEND)) != 1 {
        return;
    }
    c.vm.w8(c.st, a16!(KEY_PEND), 0);
    match c.vm.r8(c.st, a16!(KEY_CODE)) {
        0x7E => press(c, 1),
        0x7D => press(c, 2),
        0xFE | 0xFD => press(c, 0),
        _ => {}
    }
}

/// `0xA283`/`0xA299`/`0xA2BC` — latch the button word (`bx` →
/// `[0x9CDA]`); the asm also returns cursor/`bp` in regs.
fn press(c: &mut Ctx, b: u16) {
    c.vm.w16(c.st, a16!(BUTTONS), b);
}

/// Arrow-key navigation (file `0x2A20E`–`0x2A2BB`) — `+0x10..0x13` of the
/// selected record, Tandy alternates `0x29`/`0x4A`/`0x2B`/`0x4E`.
fn nav(c: &mut Ctx, code: u8) {
    let tandy = c.vm.r8(c.st, a16!(TANDY)) == 1;
    let field = match code {
        0x48 => Some(0x10),
        0x50 => Some(0x11),
        0x4B => Some(0x12),
        0x4D => Some(0x13),
        0x29 if tandy => Some(0x10),
        0x4A if tandy => Some(0x11),
        0x2B if tandy => Some(0x12),
        0x4E if tandy => Some(0x13),
        _ => None,
    };
    if let Some(f) = field {
        apply_nav(c, f);
    }
}

/// `0xA2AF` — `al = [sel·0x14 + list + field]`; `0xFF` keeps the
/// selection.
fn apply_nav(c: &mut Ctx, field: u16) {
    let sel = c.vm.r16(c.st, a16!(HOT_SEL));
    let list = c.vm.r16(c.st, a16!(HOT_LIST));
    let si = list.wrapping_add(sel.wrapping_mul(HOT_STRIDE));
    let al = c.vm.r8(c.st, si + field);
    if al != 0xFF {
        c.vm.w16(c.st, a16!(HOT_SEL), u16::from(al));
    }
}

/// `0xA2D2` — `K` mode idle: ease the cursor ⅛ toward the selected
/// hotspot's centre each frame, `y` clamped to `0xBC`.
fn snap(c: &mut Ctx) {
    let sel = c.vm.r16(c.st, a16!(HOT_SEL));
    let list = c.vm.r16(c.st, a16!(HOT_LIST));
    let si = list.wrapping_add(sel.wrapping_mul(HOT_STRIDE));
    let tx = centre(c.vm.r16(c.st, si), c.vm.r16(c.st, si + 4));
    let ty = centre(c.vm.r16(c.st, si + 2), c.vm.r16(c.st, si + 6)).min(Y_MAX);
    let nx = step(c.vm.r16(c.st, a16!(CUR_X)), tx);
    let ny = step(c.vm.r16(c.st, a16!(CUR_Y)), ty);
    if nx == c.vm.r16(c.st, a16!(CUR_X)) && ny == c.vm.r16(c.st, a16!(CUR_Y)) {
        return;
    }
    c.vm.w16(c.st, a16!(CUR_X), nx);
    c.vm.w16(c.st, a16!(CUR_Y), ny);
    c.host.svc(Call::Native(0xA497));
}

/// `lo + (hi − lo)/2` — the hotspot-centre midpoint.
fn centre(lo: u16, hi: u16) -> u16 {
    lo.wrapping_add(hi.wrapping_sub(lo) / 2)
}

/// One axis of the ⅛-ease — `cur + ((tgt − cur) >> 3 | 1)` when off
/// target, arithmetic shift like the asm's `sar`.
fn step(cur: u16, tgt: u16) -> u16 {
    let d = i16::from_ne_bytes(tgt.wrapping_sub(cur).to_ne_bytes());
    if d == 0 {
        return cur;
    }
    cur.wrapping_add(u16::from_ne_bytes(((d >> 3) | 1).to_ne_bytes()))
}
