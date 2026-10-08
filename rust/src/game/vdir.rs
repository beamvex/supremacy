//! Tick-dispatched UI/sound routines — `cs:0x7A45` (disaster fanfare),
//! `0x7E36` (jingle selector), `0x76DE`/`0x80F0`/`0x813D` (message and
//! sprite redraw passes for codes `0xAB..0xF8`).

use super::consts::DIFFICULTY;
use super::rec::{F_FOOD, F_FUEL};
use super::text;
use super::vhost::Call;
use super::vm::Flow;
use super::vops::{faction_rec, set_bits, uimode, unlink_ships, Ctx};

/// `ds:` cells for the message-flash save/restore (`0x76DE`/`0x80F0`).
const MSG_ON: u16 = 0x91E8;
const MSG_A: u16 = 0x91E9;
const MSG_B: u16 = 0x91EA;
/// `ds:0x91D3` — nonzero suppresses the sound sequences.
const QUIET: u16 = 0x91D3;
/// `ds:0x4FAA` — the per-UI-mode `{bx, dx}` blit-parameter table.
const DRAW_TAB: u16 = 0x4FAA;
/// `ds:0x7562` — 32 sprite flag bytes (`0x80` = pending draw).
const SPRITES: u16 = 0x7562;

/// `cs:0x7A45` — the disaster fanfare op: modal print, sound, the
/// `0x924D` flash loop, then halve the faction's food+fuel and unlink
/// its ships. Suspends (it `ret`s) after running.
pub fn disaster(c: &mut Ctx) -> Flow {
    c.vm.modal = true;
    text::enqueue(c, 0x5D7B);
    c.host.svc(Call::Menu);
    if c.vm.r8(c.st, QUIET) != 0xFF {
        c.host.svc(Call::Chan(0x8A5B, 0));
        c.host.svc(Call::Chan(0x8A3B, 8));
        c.host.svc(Call::Sound(0x15));
    }
    c.host.svc(Call::Flash);
    c.host.svc(Call::Present(0));
    c.host.svc(Call::PlanetPanel);
    text::enqueue(c, 0x5DAE);
    let rec = faction_rec(c);
    halve(c, rec + u16::try_from(F_FOOD).unwrap_or(0));
    halve(c, rec + u16::try_from(F_FUEL).unwrap_or(0));
    set_bits(c, 0x9146, 1);
    unlink_ships(c, rec);
    Flow::Ret
}

fn halve(c: &mut Ctx, a: u16) {
    let v = c.vm.r16(c.st, a) >> 1;
    c.vm.w16(c.st, a, v);
}

/// `cs:0x7E36` — the jingle selector (tick `0xA8`): `cs:0x37C7` picks a
/// tune index, the per-difficulty tables at `ds:0x4E96`/`0x4E4E` yield
/// the new `[0x91A6]` and the note word for `[0x91FF]`; UI mode 0
/// mirrors `[0x91A6]` into `[0x928B]` and refreshes.
pub fn jingle(c: &mut Ctx) -> Flow {
    let i = (c.host.svc(Call::Native(0x37C7)) & 0xFF) * 4;
    let tab = if c.vm.r8(c.st, u16::try_from(DIFFICULTY).unwrap_or(0)) == 0 {
        0x4E96
    } else {
        0x4E4E
    };
    c.vm.w16(c.st, 0x91A6, c.vm.r16(c.st, tab + i));
    let t = c.vm.r16(c.st, tab + i + 2);
    if t != c.vm.r16(c.st, 0x91FF) {
        c.vm.w16(c.st, 0x91FF, t);
        jingle_note(c, t);
    }
    if uimode(c) == 0 {
        let v = c.vm.r16(c.st, 0x91A6);
        c.vm.w16(c.st, 0x928B, v);
        c.host.svc(Call::Refresh);
    }
    Flow::Ret
}

/// The `0x7E67` note-play tail of `jingle`.
fn jingle_note(c: &mut Ctx, t: u16) {
    if t == 0 || c.vm.r8(c.st, QUIET) == 0xFF {
        c.host.svc(Call::Native(0x8A80));
        return;
    }
    c.host.svc(Call::Chan(0x8A5B, 0));
    c.host.svc(Call::Chan(0x8A3B, 0xFF));
    c.host.svc(Call::Sound(
        u8::try_from(t & 0xFF).unwrap_or(0).wrapping_sub(2),
    ));
    c.host.svc(Call::Native(0x8A80));
}

/// `cs:0x76DE` — the `0xAB..0xC0` message pass: draws one pending
/// sprite (random pick among flagged) at the `[0x4FAA]` row for the
/// current UI mode, saving `[0x91EA]` across it.
pub fn msg_op(c: &mut Ctx) -> Flow {
    let save = c.vm.r16(c.st, MSG_B);
    c.vm.w16(c.st, MSG_A, save);
    if c.vm.r8(c.st, MSG_ON) == 0 || uimode(c) == 0 {
        return clear_msg(c);
    }
    c.vm.w8(c.st, MSG_B, 0);
    let row = u16::from(uimode(c)) * 4 + DRAW_TAB;
    msg_pick(c, c.vm.r16(c.st, row), c.vm.r16(c.st, row + 2));
    restore(c);
    Flow::Ret
}

/// The `0x7710` pick-and-blit: retry a `rand & 0x1F` index below `0x16`
/// until its `0x7562` flag is set, clear `0x80`, blit `bx + idx`.
fn msg_pick(c: &mut Ctx, bx: u16, dx: u16) {
    loop {
        let i = c.rng.next16() & 0x1F;
        if i >= 0x16 {
            continue;
        }
        let a = SPRITES + i;
        let f = c.vm.r8(c.st, a);
        if f & 0x80 == 0 {
            continue;
        }
        c.vm.w8(c.st, a, f & 0x7F);
        c.host.svc(Call::Blit(bx + i, dx));
        return;
    }
}

/// `cs:0x80F0` — the `0xC1..0xF7` redraw pass: same save/restore
/// around a `cs:0x6709` print-interpreter call alternating sprite sets
/// `0x7562`/`0x68BF` on the tick parity.
pub fn redraw(c: &mut Ctx) -> Flow {
    let save = c.vm.r16(c.st, MSG_B);
    c.vm.w16(c.st, MSG_A, save);
    if c.vm.r8(c.st, MSG_ON) == 0 || uimode(c) == 0 {
        return clear_msg(c);
    }
    c.vm.w8(c.st, MSG_B, 0);
    let set = if c.vm.r8(c.st, 0x91CE) & 1 == 0 {
        0x7562
    } else {
        0x68BF
    };
    c.host.svc(Call::Ui(set));
    restore(c);
    Flow::Ret
}

/// `cs:0x813D` — the `0xF8` code and the shared clear tail.
pub fn clear_msg(c: &mut Ctx) -> Flow {
    c.vm.w8(c.st, MSG_ON, 0);
    let v = c.vm.r16(c.st, MSG_A);
    c.vm.w16(c.st, MSG_B, v);
    Flow::Ret
}

/// `[0x91EA] = [0x91E9]` — the shared `0x7736`/`0x8132` restore tail.
fn restore(c: &mut Ctx) {
    let v = c.vm.r16(c.st, MSG_A);
    c.vm.w16(c.st, MSG_B, v);
}
