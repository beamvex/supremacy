//! The colony-spawn event — `cs:0x7EA2` (tick `0xFE`) and the
//! `cs:0x7FA5` random-name generator it uses.

use super::consts::{DAY, DIFFICULTY, DIRTY0, REC_BASE, REC_TOTAL};
use super::rec::{F_CASH, F_OWNER};
use super::vhost::Call;
use super::vm::Flow;
use super::vops::{set_bits, uimode, Ctx};

/// `ds:` cells for the arming countdown and the armed record.
const ARM: u16 = 0x91C7;
const SPAWN: u16 = 0x9178;
/// Letter tables the name generator reads (`cs:0x7FC7`/`0x7FDD`/
/// `0x7FF1`); the digraph table's entries are 4 bytes.
const TAB1: u16 = 0x794B;
const TAB2: u16 = 0x795B;
const DIGRAPH: u16 = 0x7963;

/// `cs:0x7EA2` — arms on a `kind 4` (destroyed) record once `day` passes
/// `0x7DB`/`0x7DD` (easy/normal+), counts down `[0x91C7]`, then spawns a
/// `kind 7` colony with rolled resources and a generated name.
pub fn uprising(c: &mut Ctx) -> Flow {
    if c.vm.r8(c.st, ARM) == 0 {
        return arm(c);
    }
    let t = c.vm.r8(c.st, ARM) - 1;
    c.vm.w8(c.st, ARM, t);
    // The nonzero branch `jmp`s to `cs:0x3F17` — a bare `ret`.
    if t == 0 {
        spawn(c);
    }
    Flow::Ret
}

/// The `0x7F55` arming pass — arm on the first `kind 4` (destroyed)
/// record and start the `+0x1A/0xFA + 0xB` countdown.
fn arm(c: &mut Ctx) -> Flow {
    let day = c.vm.r16(c.st, u16::try_from(DAY).unwrap_or(0));
    let min = if c.vm.r8(c.st, u16::try_from(DIFFICULTY).unwrap_or(0)) == 0 {
        0x7DB
    } else {
        0x7DD
    };
    if day < min {
        return Flow::Ret;
    }
    if let Some(r) = find_dead(c) {
        c.vm.w16(c.st, SPAWN, r);
        c.vm.w16(c.st, r + 0x0C, 1);
        let v = c.vm.r16(c.st, r + 0x1A) / 0xFA + 0xB;
        c.vm.w8(c.st, ARM, u8::try_from(v & 0xFF).unwrap_or(0));
    }
    Flow::Ret
}

/// The `0x7F6D` scan — first `kind 4` record past rec0.
fn find_dead(c: &mut Ctx) -> Option<u16> {
    let base = c.vm.r16(c.st, u16::try_from(REC_BASE).unwrap_or(0));
    let total = c.vm.r16(c.st, u16::try_from(REC_TOTAL).unwrap_or(0));
    (1..total)
        .map(|i| base + i * 0x3A)
        .find(|&r| c.vm.r16(c.st, r + 0x0C) == 4)
}

/// The `0x7EB5` spawn — owner `{1..5}`, cash stolen from the faction,
/// rolled stocks, `kind 7`, generated name.
fn spawn(c: &mut Ctx) {
    let r = c.vm.r16(c.st, SPAWN);
    set_bits(c, u16::try_from(DIRTY0).unwrap_or(0), 0x8);
    c.vm.w16(c.st, r + 0x0C, 7);
    owner(c, r);
    steal(c, r);
    roll(c, r);
    for (ofs, v) in [(0x1E, 0x19u8), (0x1D, 0x1E), (0x21, 0x0F), (0x1F, 0x4B)] {
        c.vm.w8(c.st, r + ofs, v);
    }
    name_gen(c, r);
    if uimode(c) == 0 {
        c.host.svc(Call::PlanetPanel);
    }
}

/// `0x7EC2` — reroll `rand & 7` until it lands in `1..6`.
fn owner(c: &mut Ctx, r: u16) {
    let mut o = c.rng.next16() & 7;
    while o == 0 || o >= 6 {
        o = c.rng.next16() & 7;
    }
    c.vm.w8(
        c.st,
        r + u16::try_from(F_OWNER).unwrap_or(0),
        u8::try_from(o).unwrap_or(0),
    );
}

/// `0x7ED6` — move `rand(0x4B0)` cash from rec0 if it can cover it.
fn steal(c: &mut Ctx, r: u16) {
    let amt = c.rng.range(0x4B0);
    let r0 = c.vm.r16(c.st, u16::try_from(REC_BASE).unwrap_or(0));
    if amt < c.vm.r16(c.st, r0 + u16::try_from(F_CASH).unwrap_or(0)) {
        let v = c.vm.r16(c.st, r0 + u16::try_from(F_CASH).unwrap_or(0)) - amt;
        c.vm.w16(c.st, r0 + u16::try_from(F_CASH).unwrap_or(0), v);
        c.vm.w16(c.st, r + u16::try_from(F_CASH).unwrap_or(0), amt);
    }
}

/// `0x7EEB`–`0x7F37` — the rolled colony stocks.
fn roll(c: &mut Ctx, r: u16) {
    let rolls: [(usize, u16, u16); 6] = [
        (0x2A, 0x1194, 0),
        (0x2C, 0x2BC, 0x12C),
        (0x30, 0x3E8, 0xC8),
        (0x32, 0x384, 0x15E),
        (0x34, 0x2BC, 0x64),
        (0x36, 0x7D0, 0x1388),
    ];
    for (ofs, range, base) in rolls {
        let v = c.rng.range(range).wrapping_add(base);
        c.vm.w16(c.st, r + u16::try_from(ofs).unwrap_or(0), v);
    }
}

/// `cs:0x7FA5` — random name: 9 spaces, `rand&3 + 1` letters
/// alternating the `0x794B`/`0x795B` tables, then a 3-char digraph from
/// `0x7963` whose first letter differs from the last written.
pub fn name_gen(c: &mut Ctx, rec: u16) {
    let si = rec + u16::try_from(super::rec::F_NAME).unwrap_or(0);
    for k in 0..9 {
        c.vm.w8(c.st, si + k, 0x20);
    }
    let end = letters(c, si);
    digraph(c, end);
}

/// `0x7FC7`/`0x7FDD` — the alternating-letter loop; returns the cursor.
fn letters(c: &mut Ctx, si: u16) -> u16 {
    let mut cx = i32::from(c.rng.next16() & 3);
    let (mut i, mut tab) = (si, TAB1);
    loop {
        let m = if tab == TAB1 { 0xF } else { 7 };
        let ch = c.vm.r8(c.st, tab + (c.rng.next16() & m));
        c.vm.w8(c.st, i, ch);
        i += 1;
        cx -= 1;
        if cx < 0 {
            return i;
        }
        tab = if tab == TAB1 { TAB2 } else { TAB1 };
    }
}

/// The `0x7FF1` digraph pick — a 4-byte record whose first byte isn't
/// the name's last char; writes 3 chars.
fn digraph(c: &mut Ctx, dst: u16) {
    let prev = c.vm.r8(c.st, dst - 1);
    loop {
        let src = DIGRAPH + (c.rng.next16() & 0xF) * 4;
        if c.vm.r8(c.st, src) != prev {
            let pair = c.vm.r16(c.st, src);
            c.vm.w16(c.st, dst, pair);
            let tail = c.vm.r8(c.st, src + 2);
            c.vm.w8(c.st, dst + 2, tail);
            return;
        }
    }
}
