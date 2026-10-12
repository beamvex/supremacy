//! The named-field editor wrappers — `cs:0xE77D` (file `0x2E77D`–
//! `0x2E7CC`) prints the `0x9136` name template, prefills the target
//! with it, bumps the template's two-digit serial, then runs the
//! `0xE66E` editor bracketed with `[0x9CC5] = 1`; `cs:0xE7CD` (file
//! `0x2E7CD`–`0x2E82C`) is the same shape over the `[0x9154]` type
//! record's `+0x22` name and bumps the serial only on accept.
//! `cs:0xE82D` (file `0x2E82D`–`0x2E851`) is the serial-bump helper.

use super::cells::{IN_DONE, LINE_MODE, NAME_LEN, NAME_MAX};
use super::consts::TYPE_REC;
use super::linein::line_input;
use super::text;
use super::vops::Ctx;

/// `u16` casts for the `ds:` slot constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// The `0x9136` serialised unit-name template (file `0x2E780`).
const NAME_TPL: u16 = 0x9136;
/// The 9-byte prefill width (4 words + the trailing byte).
const NAME_W: u16 = 9;

/// `cs:0xE77D` — template prompt + prefill, `NAME_LEN = NAME_MAX`,
/// then the bracketed editor at `si+9`/`bx+9`/`bp-9`. The template
/// serial bumps *before* the edit so the next launch gets the next
/// number (file `0x2E77D`–`0x2E7CC`).
pub fn prompt_edit(c: &mut Ctx, si: u16, bx: u16, dx: u16, bp: i16) {
    text::print_str(c, NAME_TPL, bx, dx);
    copy9(c, NAME_TPL, si);
    serial_bump(c, NAME_TPL.wrapping_add(8));
    let len = c.vm.r8(c.st, a!(NAME_MAX));
    c.vm.w8(c.st, a!(NAME_LEN), len);
    edit(c, si, bx, bp);
}

/// `cs:0xE7CD` — type-record prompt + prefill, `NAME_LEN = 9`, the
/// bracketed editor, then the serial bump gated on `[0x4FCF] != 0xFF`
/// — the caller passes `cx` where the routine consumes `bp` (the
/// original's stale-`bp` quirk; the port takes `bp` in) (file
/// `0x2E7CD`–`0x2E82C`).
pub fn type_edit(c: &mut Ctx, si: u16, bx: u16, dx: u16, bp: i16) {
    let t = c.vm.r16(c.st, a!(TYPE_REC)).wrapping_add(0x22);
    text::print_str(c, t, bx, dx);
    copy9(c, t, si);
    c.vm.w8(c.st, a!(NAME_LEN), 9);
    edit(c, si, bx, bp);
    if c.vm.r8(c.st, a!(IN_DONE)) != 0xFF {
        serial_bump(c, t.wrapping_add(8));
    }
}

/// The `+9` cursor advance and the `[0x9CC5]` bracket around
/// `cs:0xE66E` shared by both wrappers.
fn edit(c: &mut Ctx, si: u16, bx: u16, bp: i16) {
    c.vm.w8(c.st, a!(LINE_MODE), 1);
    line_input(c, si.wrapping_add(NAME_W), bx.wrapping_add(NAME_W), bp - 9);
    c.vm.w8(c.st, a!(LINE_MODE), 0);
}

/// Nine bytes `di → si` — four words plus the trailing byte (the
/// `mov ax,[di]`/`mov al,[di+8]` copy block in both wrappers).
pub(crate) fn copy9(c: &mut Ctx, di: u16, si: u16) {
    for o in [0u16, 2, 4, 6] {
        let v = c.vm.r16(c.st, di.wrapping_add(o));
        c.vm.w16(c.st, si.wrapping_add(o), v);
    }
    let v = c.vm.r8(c.st, di.wrapping_add(8));
    c.vm.w8(c.st, si.wrapping_add(8), v);
}

/// `cs:0xE82D` — increment the `bl` serial digit at `[di]`; on
/// `':'` wrap to `'0'` and carry into `[di-1]` (with `'.'` treated
/// as the rollover marker) (file `0x2E82D`–`0x2E851`).
fn serial_bump(c: &mut Ctx, di: u16) {
    let mut bl = c.vm.r8(c.st, di).wrapping_add(1);
    let mut dl = c.vm.r8(c.st, di - 1);
    if bl != 0x3A {
        c.vm.w8(c.st, di, bl);
        c.vm.w8(c.st, di - 1, dl);
        return;
    }
    bl = 0x30;
    if dl == 0x2E {
        dl = 0x30;
    }
    dl = dl.wrapping_add(1);
    if dl == 0x3A {
        dl = 0x2E;
    }
    c.vm.w8(c.st, di, bl);
    c.vm.w8(c.st, di - 1, dl);
}
