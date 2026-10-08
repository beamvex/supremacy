//! The status-line ticker — `cs:0x6B8F` (file `0x36B8F`–`0x36C26`),
//! run once per shell frame while the blink timer `[0x91D1]` is idle.
//!
//! `[0x915C]` is a cursor into a `ds:0x928D`-based stream of 4-byte
//! `{msg-cursor, unused}` records ending in a `0xFE` sentinel (wraps
//! to `0x928D`). The msg cursor advances one byte per step through
//! the message text; opcode bytes:
//!
//! - `0xFC` — set `[0x91D2]` from the next byte (per-step delay).
//! - `0xFD` — arm `[0x91D1]` (blink pause) and reset `[0x91C0]` to
//!   `0x29` (x cursor for the next line).
//! - `0xFB` — print the next *word* as a number at `([0x91C0],0xB8)`
//!   via `0x698F`, then run the `0xFF` end-of-record step.
//! - `0xFF` — end of record: clear `[0x91E8]`, wrap on `0xFE`,
//!   decrement `[0x91C2]`, commit the record cursor.
//! - else — print the char at `([0x91C0]++,0xB8)` through `[0x1268]`.

use super::cells::{BLINK_T, TICK_BASE, TICK_FLAG, TICK_LEFT, TICK_PTR, TICK_WAIT, TICK_X};
use super::vhost::Call;
use super::vops::Ctx;

/// `u16` casts for the `ds:` slot constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// Status-line y coordinate (file `0x36C04`).
const ROW: u16 = 0xB8;
/// Line-start x the `0xFD` op restores (file `0x36BE6`).
const LINE_X: u16 = 0x29;

/// `cs:0x6B8F` — one ticker step (file `0x36B8F`–`0x36BDD`).
pub fn ticker_step(c: &mut Ctx) {
    if c.vm.r16(c.st, a!(TICK_LEFT)) == 0 {
        return;
    }
    let w = c.vm.r8(c.st, a!(TICK_WAIT));
    if w != 0 {
        c.vm.w8(c.st, a!(TICK_WAIT), w - 1);
        return;
    }
    let bp = c.vm.r16(c.st, a!(TICK_PTR));
    let mut di = c.vm.r16(c.st, bp);
    let bp = bp.wrapping_add(4);
    let op = c.vm.r8(c.st, di);
    di += 1;
    dispatch(c, op, di, bp);
}

/// The opcode fan-out (file `0x36BB4`–`0x36C26`).
fn dispatch(c: &mut Ctx, op: u8, di: u16, bp: u16) {
    match op {
        0xFC => {
            c.vm.w8(c.st, a!(TICK_WAIT), c.vm.r8(c.st, di));
            c.vm.w16(c.st, bp - 4, di + 1);
        }
        0xFD => {
            c.vm.w8(c.st, a!(BLINK_T), 1);
            c.vm.w16(c.st, bp - 4, di);
            c.vm.w16(c.st, a!(TICK_X), LINE_X);
        }
        0xFB => num(c, di, bp),
        0xFF => end_rec(c, bp),
        ch => {
            c.vm.w16(c.st, bp - 4, di);
            let x = c.vm.r16(c.st, a!(TICK_X));
            c.vm.w16(c.st, a!(TICK_X), x + 1);
            c.host.svc(Call::Glyph(ch, x, ROW));
        }
    }
}

/// `0xFB` — end-of-record step then print the word at `di` via
/// `0x698F` (its `bx` advances one column per decimal digit, file
/// `0x3698F`–`0x369E5`).
fn num(c: &mut Ctx, di: u16, bp: u16) {
    end_rec(c, bp);
    let v = c.vm.r16(c.st, di);
    let x = c.vm.r16(c.st, a!(TICK_X));
    c.host.svc(Call::Num(u32::from(v), x, ROW));
    c.vm.w16(c.st, a!(TICK_X), x + u16::from(digits(v)));
}

/// Decimal digit count of `v` as `0x698F` prints it (min 1).
fn digits(v: u16) -> u8 {
    let mut n = 0u8;
    let mut v = v;
    loop {
        n += 1;
        v /= 10;
        if v == 0 {
            return n;
        }
    }
}

/// `cs:0x6BC4` — end-of-record: clear the tick flag, wrap the record
/// cursor on a `0xFE` sentinel, count down `[0x91C2]` (file
/// `0x36BC4`–`0x36BDB`).
fn end_rec(c: &mut Ctx, bp: u16) {
    c.vm.w8(c.st, a!(TICK_FLAG), 0);
    let next = if c.vm.r16(c.st, bp) == 0xFE {
        a!(TICK_BASE)
    } else {
        bp
    };
    let left = c.vm.r16(c.st, a!(TICK_LEFT)).wrapping_sub(1);
    c.vm.w16(c.st, a!(TICK_LEFT), left);
    c.vm.w16(c.st, a!(TICK_PTR), next);
}
