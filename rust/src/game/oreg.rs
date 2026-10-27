//! The `[0x1298]` dirty-region register and `cs:0xA479` release —
//! cursor-overlap bookkeeping the paint blocks bracket their edits
//! with (MCG slot body `cs:0x6324`, file `0x2F5D4`–`0x2F628`; release
//! file `0x3A479`–`0x3A496`). Nest-counted: only the outermost pair
//! touches the rect cells and the cursor.

use super::consts::{CUR_X, CUR_Y, REG_HIT, REG_NEST, REG_ON, REG_X1, REG_X2, REG_Y1, REG_Y2};
use super::vhost::Call;
use super::vops::Ctx;

macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// `cs:0x6324` — bump `[0x4C94]`; nested calls only count. On the
/// outermost call with the cursor live (`[0x9CCE]`) and inside
/// `x1−16..x2`/`y1−16..y2`, hide it (`0xA3BC`) and mark `[0x9CD0]`;
/// the rect lands in `[0x9CDE]`/`[0x9CE0]`/`[0x9CE2]`/`[0x9CDC]`.
pub fn push(c: &mut Ctx, ax: u16, bx: u16, cx: u16, dx: u16) {
    let n = c.vm.r8(c.st, a!(REG_NEST)).wrapping_add(1);
    c.vm.w8(c.st, a!(REG_NEST), n);
    if n > 1 {
        return;
    }
    if c.vm.r16(c.st, a!(REG_ON)) != 0 && covers(c, ax, bx, cx, dx) {
        c.host.svc(Call::Native(0xA3BC));
        c.vm.w16(c.st, a!(REG_HIT), 1);
    }
    c.vm.w16(c.st, a!(REG_X1), ax);
    c.vm.w16(c.st, a!(REG_X2), bx);
    c.vm.w16(c.st, a!(REG_Y1), cx);
    c.vm.w16(c.st, a!(REG_Y2), dx);
}

/// The `0x2F5E8`–`0x2F616` inside test — cursor (with its 16px slack)
/// overlapping the rect.
fn covers(c: &mut Ctx, ax: u16, bx: u16, cx: u16, dx: u16) -> bool {
    let mx = c.vm.r16(c.st, a!(CUR_X));
    let my = c.vm.r16(c.st, a!(CUR_Y));
    mx.wrapping_add(0x10) >= ax && bx >= mx && my.wrapping_add(0x10) >= cx && dx >= my
}

/// `cs:0xA479` — `[0x4C94]` dec; at zero, a `[0x9CD0]` hide is
/// answered by `0xA36A`, then `[0x9CD0]` clears and `[0x9CE2]` goes
/// to `0xFFFF`.
pub fn pop(c: &mut Ctx) {
    let n = c.vm.r8(c.st, a!(REG_NEST)).wrapping_sub(1);
    c.vm.w8(c.st, a!(REG_NEST), n);
    if n != 0 {
        return;
    }
    if c.vm.r16(c.st, a!(REG_HIT)) == 1 {
        c.host.svc(Call::Native(0xA36A));
    }
    c.vm.w16(c.st, a!(REG_HIT), 0);
    c.vm.w16(c.st, a!(REG_Y1), 0xFFFF);
}
