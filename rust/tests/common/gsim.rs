//! Game-sim helpers: a preset galaxy, record/machine/ship offsets, and a
//! sequencer assertion for `tick_step`.

use supremacy::game::{self, Rng, State, Tick};

/// A fresh galaxy (preset 3: `WOTOK`, 8 records).
pub fn galaxy() -> State {
    let mut st = State::new();
    game::select_galaxy(&mut st, 3);
    st
}

/// Planet `i` record offset in `st`.
pub fn rec(st: &State, i: u16) -> usize {
    usize::from(st.rec_ofs(i))
}

/// Machine `i` record offset.
pub fn mach(i: u16) -> usize {
    game::MACH_BASE + usize::from(i) * game::MACH_STRIDE
}

/// Ship `i` record offset.
pub fn ship(i: u16) -> usize {
    game::FLEET_BASE + usize::from(i) * game::FLEET_STRIDE
}

/// Machine `i` configured as type/flags on host planet 0.
pub fn set_mach(st: &mut State, i: u16, ty: u8, flags: u8) -> usize {
    let m = mach(i);
    st.set_byte(m + game::M_TYPE, ty);
    st.set_byte(m + game::M_FLAGS, flags);
    st.set_byte(m + game::M_HOST, 0);
    m
}

/// One `tick_step` that must dispatch to arm `want`.
pub fn step(st: &mut State, rng: &mut Rng, want: char) -> Tick {
    let mut vm = game::Vm::new();
    let mut host = game::NullHost;
    let got = {
        let mut c = game::Ctx {
            vm: &mut vm,
            st,
            rng,
            host: &mut host,
        };
        game::tick_step(&mut c)
    };
    let ok = matches!(
        (want, &got),
        ('m', Tick::Machine(_))
            | ('p', Tick::Planet(_))
            | ('d', Tick::Defence(_))
            | ('y', Tick::Day)
            | ('?', Tick::Pending(_))
            | ('s', Tick::Script(_))
    );
    assert!(ok, "expected arm {want}, got {got:?}");
    got
}
