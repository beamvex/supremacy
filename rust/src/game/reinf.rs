//! The reinforcement recolonise tail of the defence pass — the
//! `0x34911`–`0x3496F` block of `cs:0x489D` plus its `cs:0x8019`
//! machine-spawn helper (`0x803A` per unit, `0x6611` slot scan).
//!
//! When a planet's ground troops are gone but armed ships still cover
//! it (the `0x34550` `jmp 0x489D`, or the repelled pass's troop
//! underflow) and its kind isn't already `0xA`, the crews recolonise
//! it: `1..=5` machines spawn on the record, the report pair enqueues,
//! the kind becomes `0xA`, the orders flag arms and — when the record
//! is the selected one — the panel redraws through `0x5C97`. Flipping
//! the record back to inhabited is what the `0xFA` endgame monitor
//! counts for its victory arm, so this tail is half the win path.

use super::consts::{
    MACH_BASE, MACH_COUNT, MACH_STRIDE, SCAN_REC, SEL_REC, SND_ON, TYPE_BASE, TYPE_STRIDE,
};
use super::mach::{
    M_ACC, M_BUILD, M_DEPOSIT, M_FLAGS, M_HOST, M_OPS, M_TIMER, M_TYPE, T_14, T_BUILD, T_DEPOSIT,
    T_RATE, T_TYPE,
};
use super::ppanel;
use super::rec::{F_KIND, F_NAME, F_OWNER, F_SERIAL};
use super::text;
use super::vhost::Call;
use super::vops::Ctx;
use super::vup::name_span;

/// `ds:` slot the `0x6611` slot scan leaves full on (`[0x91E0]`, file
/// `0x36611`/`0x3662C`).
const SLOT_FULL: usize = 0x91E0;
/// `ds:` message-name scratch the report prints (`0x8478`, file
/// `0x3491B`).
const MSG_NAME: usize = 0x8478;
/// `ds:` report strings enqueued on the ticker (file `0x3492C`/`0x34932`).
const MSG_A: u16 = 0x841D;
const MSG_B: u16 = 0x845F;
/// `ds:` orders-popup flag re-armed on recolonise (`[0x91F4]`, file
/// `0x34941`).
const ORD_FLAG: usize = 0x91F4;

/// `0x34911`–`0x3496F` — spawn the garrison machines, report the
/// recolonisation, `kind → 0xA`, arm the orders flag, play the sting
/// and redraw the panel when the record is selected.
pub(super) fn recolonise(c: &mut Ctx, r: usize) {
    spawn_machines(c);
    report(c, r);
    c.st.set_word(r + F_KIND, 0xA);
    c.st.set_byte(ORD_FLAG, 0xFF);
    sfx(c);
    if c.st.word(SCAN_REC) == c.st.word(SEL_REC) {
        ppanel::planet_panel(c);
    }
}

/// `0x34914`–`0x34937` — the 8-byte name copy into `ds:0x8478`, then
/// the `0x841D`/`0x845F` report pair on the `0x6B06` ticker.
fn report(c: &mut Ctx, r: usize) {
    for k in 0..8 {
        let b = c.st.byte(r + F_NAME + k);
        c.st.set_byte(MSG_NAME + k, b);
    }
    text::enqueue(c, MSG_A);
    text::enqueue(c, MSG_B);
}

/// `0x34946`–`0x34961` — the recolonise sting (`0x8A5B`, `al = 5`,
/// `cl = 3` on the driver); `[0x91D3] == 0xFF` mutes it.
fn sfx(c: &mut Ctx) {
    if c.st.byte(SND_ON) == 0xFF {
        return;
    }
    c.host.svc(Call::Chan(0x8A5B, 0));
    c.host.svc(Call::Chan(0x8A3B, 5));
    c.host.svc(Call::Sound(3));
}

/// `cs:0x8019` — `cx = (rand & 7) − (rand & 3)`: spawn `cx + 1`
/// machines while `0 ≤ cx < 5` (the unsigned `cl ≥ 5` test skips both
/// negative diffs and `5..=7`, so this rolls `1..=5` or nothing).
fn spawn_machines(c: &mut Ctx) {
    let n = i32::from(c.rng.next16() & 7) - i32::from(c.rng.next16() & 3);
    if !(0..5).contains(&n) {
        return;
    }
    for _ in 0..=n {
        spawn_one(c);
    }
}

/// `cs:0x803A` — fill one free machine slot bound to the scan record
/// (`M_HOST` = its serial). The full-array bail is the `0x803D` early
/// `jmp 0x80D1`; the spawn loop just calls again and no-ops the same.
fn spawn_one(c: &mut Ctx) {
    let Some(m) = free_slot(c) else { return };
    let p = usize::from(c.st.word(SCAN_REC));
    let serial = c.st.byte(p + F_SERIAL);
    c.st.set_byte(m + M_HOST, serial);
    c.st.set_byte(m + M_FLAGS, 0x30);
    let owner = c.st.byte(p + F_OWNER);
    let t = TYPE_BASE + type_entry(c, owner, m) * TYPE_STRIDE;
    fill(c, m, t);
    // `jmp 0x7FA8` — the name generator's body writes the 9-char name
    // at the machine's own `+0` field.
    name_span(c, u16::try_from(m).unwrap_or(0));
}

/// `0x3805C`–`0x38084` — table entry `6`/`5` for owners `4`/`5` (flags
/// stay the `0x30` written above); others get `4`, or `0` on `rand & 1`
/// with the flags rewritten to `0x10`.
fn type_entry(c: &mut Ctx, owner: u8, m: usize) -> usize {
    match owner {
        4 => 6,
        5 => 5,
        _ => {
            c.st.set_byte(m + M_FLAGS, 0x10);
            if c.rng.next16() & 1 == 0 {
                4
            } else {
                0
            }
        }
    }
}

/// `0x38085`–`0x380CE` — copy the type-table entry at `t` into the
/// slot: build timer, `+0x14`, ops/type/rate, rolled deposit.
fn fill(c: &mut Ctx, m: usize, t: usize) {
    let (timer, aux, ty, rate, dep) = (
        c.st.word(t + T_BUILD),
        c.st.word(t + T_14),
        c.st.byte(t + T_TYPE),
        c.st.byte(t + T_RATE),
        c.st.word(t + T_DEPOSIT),
    );
    c.st.set_word(m + M_ACC, 0);
    c.st.set_word(m + M_TIMER, timer);
    for o in [0x0E, 0x10, 0x12] {
        c.st.set_word(m + o, 0);
    }
    c.st.set_word(m + 0x14, aux);
    c.st.set_word(m + M_OPS, 0);
    c.st.set_byte(m + M_TYPE, ty);
    c.st.set_byte(m + 0x20, rate);
    c.st.set_word(m + M_BUILD, 0);
    c.st.set_word(m + 0x24, 0);
    c.st.set_word(m + M_DEPOSIT, c.rng.range(dep));
}

/// `cs:0x6611` — first machine slot with `M_TYPE == 0`; `[0x91E0] =
/// 0xFF` and `None` when all `0x20` slots are live.
fn free_slot(c: &mut Ctx) -> Option<usize> {
    c.st.set_byte(SLOT_FULL, 0);
    for i in 0..MACH_COUNT {
        let m = MACH_BASE + usize::from(i) * MACH_STRIDE;
        if c.st.byte(m + M_TYPE) == 0 {
            return Some(m);
        }
    }
    c.st.set_byte(SLOT_FULL, 0xFF);
    None
}
