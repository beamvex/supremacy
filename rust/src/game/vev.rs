//! The scripted game events — the tick-direct routines `cs:0x7C62`
//! (tribute), `0x7C9F` (pirate raid), `0x7EA2` (colony spawn) with the
//! `cs:0x7D5A` victim picker and the `cs:0x7FA5` name generator.
//! All `ret` — as script ops they suspend after firing.

use super::consts::{DIFFICULTY, REC_BASE, REC_COUNT, TICK};
use super::mach::{M_FLAGS, M_HOST, M_TYPE};
use super::rec::{F_ATTACK, F_CASH, F_KIND, F_NAME, F_POP, F_TROOPS};
use super::text;
use super::vhost::Call;
use super::vm::Flow;
use super::vops::{faction_rec, Ctx};

/// `ds:` flag cells and misc slots the event routines use.
const E7: u16 = 0x91E7; // extended-difficulty flag (raises payouts)
const CF: u16 = 0x91CF; // victim-pick failure latch
const AA: u16 = 0x91AA; // round-robin victim cursor
const RAID_NAME: u16 = 0x8410; // pirate message name scratch
const QUIET: u16 = 0x91D3;

fn f(c: &Ctx, r: u16, o: usize) -> u16 {
    c.vm.r16(c.st, r + u16::try_from(o).unwrap_or(0))
}

fn put(c: &mut Ctx, r: u16, o: usize, v: u16) {
    c.vm.w16(c.st, r + u16::try_from(o).unwrap_or(0), v);
}

fn rec0(c: &Ctx) -> u16 {
    c.vm.r16(c.st, u16::try_from(REC_BASE).unwrap_or(0))
}

fn diff(c: &Ctx) -> u8 {
    c.vm.r8(c.st, u16::try_from(DIFFICULTY).unwrap_or(0))
}

/// `cs:0x7C62` (tick `0xA9`) — tribute: while rec0 is conquered
/// (`kind 7`), unthreatened and below `0xA240` cash, add the
/// difficulty-scaled dole `10/22/34/50`.
pub fn tribute(c: &mut Ctx) -> Flow {
    let r = rec0(c);
    if f(c, r, F_KIND) != 7 || f(c, r, F_ATTACK) != 0 || f(c, r, F_CASH) >= 0xA240 {
        return Flow::Ret;
    }
    let v = match diff(c) {
        0 => 0xA,
        1 => 0x16,
        _ if c.vm.r8(c.st, E7) == 0 => 0x22,
        _ => 0x32,
    };
    put(c, r, F_CASH, f(c, r, F_CASH) + v);
    Flow::Ret
}

/// `cs:0x7C9F` (tick `0xFB`) — pirate raid: when rec0's cash passes the
/// difficulty threshold, a raider force (`+0x26` strength) lands on a
/// picked victim planet and the cash is stolen; prints `0x83F6`.
pub fn pirate(c: &mut Ctx) -> Flow {
    let r = rec0(c);
    let lim = match diff(c) {
        0 => 0x2C38,
        1 => 0x568C,
        _ => 0x7D9D,
    };
    if f(c, r, F_CASH) < lim {
        return Flow::Ret;
    }
    let v = victim(c);
    if c.vm.r8(c.st, CF) != 0 || f(c, v, F_KIND) != 0xA || f(c, v, F_TROOPS) != 0 {
        return Flow::Ret;
    }
    raid(c, r, v);
    Flow::Ret
}

/// The `0x7CEA` steal + report tail of `pirate`.
fn raid(c: &mut Ctx, r0: u16, v: u16) {
    let (mask, base) = match diff(c) {
        0 => (0x1FF, 0x78),
        1 => (0x7F, 0x37),
        _ if c.vm.r8(c.st, E7) == 0 => (0x3FF, 0x1F4),
        _ => (0x3FF, 0x4E2),
    };
    let amt = (c.rng.next16() & mask) + base;
    put(c, v, F_TROOPS, amt);
    put(c, r0, F_CASH, f(c, r0, F_CASH) - amt);
    for k in 0..9 {
        let b = c.vm.r8(c.st, v + u16::try_from(F_NAME).unwrap_or(0) + k);
        c.vm.w8(c.st, RAID_NAME + k, b);
    }
    text::enqueue(c, 0x83F6);
    raid_sfx(c);
}

/// The `0x7D42` raid sting — skipped while `[0x91D3]` is `0xFF`.
fn raid_sfx(c: &mut Ctx) {
    if c.vm.r8(c.st, QUIET) != 0xFF {
        c.host.svc(Call::Chan(0x8A5B, 0));
        c.host.svc(Call::Chan(0x8A3B, 0x1A));
        c.host.svc(Call::Sound(0x0A));
    }
}

/// `cs:0x7D5A` — pick the raid victim: gate on an inhabited planet,
/// then select by difficulty (round-robin / random / the `day % 8`
/// schedule with machine scans). Sets `[0x91CF]` on failure and
/// returns a (possibly invalid) record pointer like the asm.
fn victim(c: &mut Ctx) -> u16 {
    c.vm.w8(c.st, CF, 0);
    if first_inhabited(c).is_none() {
        return rec0(c);
    }
    let v = match diff(c) {
        0 => victim_rr(c),
        1 => Some(victim_rand(c)),
        _ => victim_hard(c),
    };
    v.unwrap_or_else(|| {
        c.vm.w8(c.st, CF, 0xFF);
        rec0(c)
    })
}

/// The `0x7D68` "any inhabited?" scan — first `kind == 0xA` planet.
fn first_inhabited(c: &Ctx) -> Option<u16> {
    let base = rec0(c);
    let n = c.vm.r16(c.st, u16::try_from(REC_COUNT).unwrap_or(0));
    (0..n)
        .map(|i| base + i * 0x3A)
        .find(|&r| f(c, r, F_KIND) == 0xA)
}

/// Difficulty-0 picker (`0x7DCE`): round-robin via `[0x91AA]`.
fn victim_rr(c: &mut Ctx) -> Option<u16> {
    let n = c.vm.r16(c.st, u16::try_from(REC_COUNT).unwrap_or(0));
    let i = c.vm.r16(c.st, AA) + 1;
    if i >= n {
        c.vm.w16(c.st, AA, 0);
        return None;
    }
    c.vm.w16(c.st, AA, i);
    Some(c.vm.rec_ofs(c.st, i))
}

/// Difficulty-1 picker (`0x7DBF`): `rand & [0x91B8]` (retry on `==`).
fn victim_rand(c: &mut Ctx) -> u16 {
    let mask = c.vm.r16(c.st, u16::try_from(REC_COUNT).unwrap_or(0));
    let mut i = c.rng.next16() & mask;
    while i == mask {
        i = c.rng.next16() & mask;
    }
    c.vm.rec_ofs(c.st, i)
}

/// Difficulty-2 picker (`0x7D83`): `tick % 8` — `0`/`1` scan machines
/// of type `7`/`6` for a powered one and raid its host; `2` picks a
/// weak (`pop < 0x1388`) inhabited planet; `3` random; `4..7` fail.
fn victim_hard(c: &mut Ctx) -> Option<u16> {
    match c.vm.r16(c.st, u16::try_from(TICK).unwrap_or(0)) & 7 {
        t @ (0 | 1) => victim_machine(c, 7 - t),
        2 => victim_weak(c),
        3 => Some(victim_rand(c)),
        _ => None,
    }
}

/// The `0x7DFC` machine scan — on the first powered type-`k` machine,
/// raid its host planet; fails outright (`0x7E13`) if the host is the
/// faction record.
fn victim_machine(c: &mut Ctx, k: u16) -> Option<u16> {
    for i in 0..0x20u16 {
        let m = u16::try_from(super::consts::MACH_BASE).unwrap_or(0) + i * 0x28;
        let on = c.vm.r8(c.st, m + u16::try_from(M_FLAGS).unwrap_or(0)) & 0x10;
        let ty = c.vm.r8(c.st, m + u16::try_from(M_TYPE).unwrap_or(0));
        if ty != u8::try_from(k).unwrap_or(0) || on == 0 {
            continue;
        }
        let r = c.vm.rec_ofs(
            c.st,
            u16::from(c.vm.r8(c.st, m + u16::try_from(M_HOST).unwrap_or(0))),
        );
        return (r != faction_rec(c)).then_some(r);
    }
    None
}

/// The `0x7D9E` weak-planet scan — inhabited with `pop < 0x1388`.
fn victim_weak(c: &mut Ctx) -> Option<u16> {
    let base = rec0(c);
    let n = c.vm.r16(c.st, u16::try_from(REC_COUNT).unwrap_or(0));
    (0..n)
        .map(|i| base + i * 0x3A)
        .find(|&r| f(c, r, F_KIND) == 0xA && f(c, r, F_POP) < 0x1388)
}
