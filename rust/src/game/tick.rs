use super::battle::Battle;
use super::consts::{MACH_COUNT, REC_TOTAL, SEQ};
use super::defence::defence_tick;
use super::fleet::{ship_tick, ShipOut};
use super::machine::{machine_tick, MachOut};
use super::sim::sim_planet;
use super::vend::{endgame, Ending};
use super::vm::Flow;
use super::vops::Ctx;
use super::{day_tick, vdir, vev, vhud, vops, vord, vup};

/// Which dispatcher arm a `tick_step` ran — mirrors the `bl` switch at
/// file `0x37619`–`0x376C5`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tick {
    /// `bl < 0x20` — machine record `bl` (file `0x37416`).
    Machine(MachOut),
    /// `0x38 + i` — planet `i` simulation; payload is the starvation
    /// warning flag.
    Planet(bool),
    /// `0x38 + N + i` — planet `i` defence pass.
    Defence(Battle),
    /// `0x38 + 2N + i` — event slot `i` ran (`cs:0x773D` VM; payload is
    /// the op count before it suspended).
    Script(u16),
    /// `0x20..0x38` — ship `bl − 0x20` tick (`cs:0x60EE`).
    Ship(ShipOut),
    /// `0xA7` — the `cs:0x60A8` selected-object reprint.
    HudSel,
    /// `0xAA` — the `cs:0x3738` mark-all-sprites pass.
    Sprites,
    /// `0xF9` — the `cs:0x4309` orders popup (payload = window opened).
    Orders(bool),
    /// `0xFA` — the `cs:0x82E5` endgame monitor.
    Endgame(Option<Ending>),
    /// `0xFC` — the `cs:0x60D4` faction-population reprint.
    HudPop,
    /// `0xFD` — the `cs:0x60B6` faction-credits reprint.
    HudCredits,
    /// `0xA8` — the `cs:0x7E36` jingle selector.
    Jingle,
    /// `0xA9` — the `cs:0x7C62` tribute dole.
    Tribute,
    /// `0xAB..0xC0` — the `cs:0x76DE` message pass.
    MsgOp,
    /// `0xC1..0xF7` — the `0x80F0` sprite redraw pass.
    Redraw,
    /// `0xF8` — `0x813D`: clear the message flag, restore `0x91EA`.
    ClearMsg,
    /// `0xFB` — the `cs:0x7C9F` pirate raid.
    Pirate,
    /// `0xFE` — the `cs:0x7EA2` colony spawn.
    Uprising,
    /// `0xFF` — day rollover.
    Day,
    /// Dispatcher codes below `0x20` — the `0x00`/`0x20`-boundary
    /// fall-through `ret` (`cs:0x76C5`); every named code is ported.
    Idle(u8),
}

/// One step of the game's tick — `cs:0x73D9` (file `0x373D9`–`0x376C5`).
///
/// `[0x91CE]` advances and selects the work: machines `0..0x20`, then
/// the `bl` switch — planets, defence, event scripts, specials and the
/// `0xFF` day rollover. Called from the main loop once per frame.
pub fn tick_step(c: &mut Ctx) -> Tick {
    sound_flag(c);
    let code = c.st.byte(SEQ).wrapping_add(1);
    c.st.set_byte(SEQ, code);
    if code < u8::try_from(MACH_COUNT).unwrap_or(0) {
        return Tick::Machine(machine_tick(c.st, u16::from(code), c.rng));
    }
    dispatch(c, code)
}

/// `[0x91B0]` nonzero → sound-driver service then clear (file
/// `0x373D9`–`0x373FD`; the `call far [0x127C]` is the shell's job).
fn sound_flag(c: &mut Ctx) {
    if c.st.word(0x91B0) != 0 {
        c.st.set_word(0x91B0, 0);
    }
}

/// The `bl` switch (file `0x37619`–`0x376C5`) — record-indexed arms
/// first, then the point codes; unmapped codes return [`Tick::Idle`]
/// (the asm `ret`s at `0x76C5`).
fn dispatch(c: &mut Ctx, bl: u8) -> Tick {
    let n = c.st.word(REC_TOTAL);
    let b = u16::from(bl);
    if b < 0x38 {
        return ship_or_idle(c, bl);
    }
    if b < 0x38 + n {
        return Tick::Planet(sim_planet(c.st, b - 0x38));
    }
    if b < 0x38 + 2 * n {
        return Tick::Defence(defence_tick(c.st, b - 0x38 - n));
    }
    if b < 0x56 + 2 * n {
        return Tick::Script(vops::run(c, b - 0x38 - 2 * n));
    }
    specials(c, bl)
}

/// `bl < 0x38` — codes `0x20..0x37` index the ship array (`jmp 0x60EE`,
/// `bx = bl − 0x20`); below `0x20` the machines arm already ran.
fn ship_or_idle(c: &mut Ctx, bl: u8) -> Tick {
    if bl < 0x20 {
        return Tick::Idle(bl);
    }
    Tick::Ship(ship_tick(c, u16::from(bl - 0x20)))
}

/// The point codes above `0x56 + 2N` (file `0x37653`–`0x376C2`).
fn specials(c: &mut Ctx, bl: u8) -> Tick {
    match bl {
        0xA7 => hud(c, vhud::hud_ship, Tick::HudSel),
        0xAA => hud(c, vhud::mark_sprites, Tick::Sprites),
        0xF9 => Tick::Orders(vord::orders(c)),
        0xFA => Tick::Endgame(endgame(c)),
        0xFC => hud(c, vhud::hud_pop, Tick::HudPop),
        0xFD => hud(c, vhud::hud_credits, Tick::HudCredits),
        _ => flow_arm(c, bl),
    }
}

/// The [`Flow`]-returning point codes (file `0x3765B`–`0x376C2`).
fn flow_arm(c: &mut Ctx, bl: u8) -> Tick {
    let (f, t): (fn(&mut Ctx) -> Flow, Tick) = match bl {
        0xA8 => (vdir::jingle, Tick::Jingle),
        0xA9 => (vev::tribute, Tick::Tribute),
        0xAB..=0xC0 => (vdir::msg_op, Tick::MsgOp),
        0xC1..=0xF7 => (vdir::redraw, Tick::Redraw),
        0xF8 => (vdir::clear_msg, Tick::ClearMsg),
        0xFB => (vev::pirate, Tick::Pirate),
        0xFE => (vup::uprising, Tick::Uprising),
        0xFF => return day(c),
        _ => return Tick::Idle(bl),
    };
    f(c);
    t
}

/// The `() -> ()` HUD helpers — run and report the arm's tag.
fn hud(c: &mut Ctx, f: fn(&mut Ctx), t: Tick) -> Tick {
    f(c);
    t
}

/// `0xFF` — the day-rollover arm (file `0x376BC`).
fn day(c: &mut Ctx) -> Tick {
    day_tick(c.st);
    Tick::Day
}
