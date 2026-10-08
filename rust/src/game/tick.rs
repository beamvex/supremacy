use super::battle::Battle;
use super::consts::{MACH_COUNT, REC_TOTAL, SEQ};
use super::defence::defence_tick;
use super::machine::{machine_tick, MachOut};
use super::sim::sim_planet;
use super::{day_tick, Rng, State};

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
    /// `0x38 + 2N + i` — event-script op `i` (`cs:0x773D`; interpreter
    /// pending).
    Script(u16),
    /// `0xFF` — day rollover.
    Day,
    /// `0x20..0x38` (`cs:0x60EE`), `0xA7`–`0xAA` (`0x60A8`/`0x7E36`/
    /// `0x7C62`/`0x3738`), `0xAB`–`0xC0` (`0x76DE` UI tasks),
    /// `0xC1`–`0xF7` (`0x80F0`), `0xF8`–`0xFE` (`0x813D`/`0x4309`/
    /// `0x82E5`/`0x7C9F`/`0x60D4`/`0x60B6`/`0x7EA2`) — decoded ranges
    /// not yet ported.
    Pending(u8),
}

/// One step of the game's tick — `cs:0x73D9` (file `0x373D9`–`0x376C5`).
///
/// `[0x91CE]` advances and selects the work: machines `0..0x20`, then
/// the `bl` switch — planets, defence, event scripts, specials and the
/// `0xFF` day rollover. Called from the main loop once per frame.
pub fn tick_step(st: &mut State, rng: &mut Rng) -> Tick {
    sound_flag(st);
    let code = st.byte(SEQ).wrapping_add(1);
    st.set_byte(SEQ, code);
    if code < u8::try_from(MACH_COUNT).unwrap_or(0) {
        return Tick::Machine(machine_tick(st, u16::from(code), rng));
    }
    dispatch(st, code)
}

/// `[0x91B0]` nonzero → sound-driver service then clear (file
/// `0x373D9`–`0x373FD`; the `call far [0x127C]` is the shell's job).
fn sound_flag(st: &mut State) {
    if st.word(0x91B0) != 0 {
        st.set_word(0x91B0, 0);
    }
}

/// The `bl` switch (file `0x37619`–`0x376C5`) — record-indexed arms
/// first, then the point codes; unmapped codes return [`Tick::Pending`]
/// (the asm `ret`s at `0x76C5`).
fn dispatch(st: &mut State, bl: u8) -> Tick {
    let n = st.word(REC_TOTAL);
    let b = u16::from(bl);
    if b < 0x38 {
        return Tick::Pending(bl);
    }
    if b < 0x38 + n {
        return Tick::Planet(sim_planet(st, b - 0x38));
    }
    if b < 0x38 + 2 * n {
        return Tick::Defence(defence_tick(st, b - 0x38 - n));
    }
    if b < 0x56 + 2 * n {
        return Tick::Script(b - 0x38 - 2 * n);
    }
    if bl == 0xFF {
        day_tick(st);
        Tick::Day
    } else {
        Tick::Pending(bl)
    }
}
