use super::consts::{
    DIRTY0, DIRTY1, G_FARMTECH, G_MINETECH, MACH_BASE, MACH_STRIDE, MSG_BUILD, QUEUED_REC,
};
use super::mach::{M_BUILD, M_FLAGS, M_HOST, M_TIMER, M_TYPE};
use super::mach::{TYPE_FARM, TYPE_MINER, TYPE_SOLAR};
use super::mine::mine;
use super::rec::{F_CREDITS, F_ENERGY, F_FOOD, F_FUEL, F_KIND, F_MINERALS, F_OWNER, F_POP};
use super::{Rng, State};

/// Stock ceiling shared by all production (file `0x3746B`).
const STOCK_MAX: u16 = 0x7530;

/// What a machine tick did — the shell cares about the UI side-effects.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MachOut {
    /// Inactive slot or plain countdown — nothing to report.
    Quiet,
    /// Production/mining ran.
    Ran,
    /// `M_BUILD` hit zero — the queued planet was colonised (asm also
    /// prints the message at `ds:0x831A` and jumps to `0x5C97`).
    Colonised,
}

/// One machine record's tick — `cs:0x742C` region (file `0x37423`–
/// `0x375C1`). `idx` is the `[0x91CE]` sequencer value `0..0x20`.
pub fn machine_tick(st: &mut State, idx: u16, rng: &mut Rng) -> MachOut {
    let m = MACH_BASE + usize::from(idx) * MACH_STRIDE;
    if st.byte(m + M_TYPE) == 0 {
        return MachOut::Quiet;
    }
    countdown(st, m);
    if st.byte(m + M_FLAGS) & 4 == 0 {
        produce(st, m);
    }
    solar(st, m);
    mine(st, m);
    build(st, m, rng)
}

/// `+0xC` countdown → dirty `0x800` (file `0x3742C`–`0x3743D`).
fn countdown(st: &mut State, m: usize) {
    let t = st.word(m + M_TIMER);
    if t != 0 {
        st.set_word(m + M_TIMER, t - 1);
        st.set_word(DIRTY0, st.word(DIRTY0) | 0x800);
    }
}

/// Types 6/7 production (file `0x37446`–`0x37504`) — skipped while flag
/// `0x4` is set (the asm jumps straight to the solar check).
fn produce(st: &mut State, m: usize) {
    let ty = st.byte(m + M_TYPE);
    let online = st.byte(m + M_FLAGS) & 0x20 != 0;
    if online && ty == TYPE_MINER {
        miner(st, m);
    } else if online && ty == TYPE_FARM {
        farm(st, m);
    }
}

/// `si` → the machine's host planet record offset (`call 0x5085`).
fn host(st: &State, m: usize) -> usize {
    let idx = u16::from(st.byte(m + M_HOST));
    usize::from(st.rec_ofs(idx))
}

/// Dirty-bit helper for `+0x9144`.
fn dirty(st: &mut State, bits: u16) {
    st.set_word(DIRTY0, st.word(DIRTY0) | bits);
}

/// Core-miner: `+2/+7` minerals, `+7/+0x19/+0xF` fuel (file `0x37468`–
/// `0x374B7`).
fn miner(st: &mut State, m: usize) {
    let p = host(st, m);
    let (tech, owner5) = (st.byte(G_MINETECH) != 0, st.byte(p + F_OWNER) == 5);
    let mut add = |f: usize, base: u16, t: u16, o: u16| {
        let cur = st.word(p + f);
        if cur < STOCK_MAX {
            let v = base + u16::from(tech) * t + u16::from(owner5) * o;
            st.set_word(p + f, cur + v);
        }
    };
    add(F_MINERALS, 2, 7, 5);
    add(F_FUEL, 7, 0x19, 0xF);
    st.set_word(DIRTY1, st.word(DIRTY1) | 0x3);
    dirty(st, 0x8000);
}

/// Horticultural: burns 1 energy else goes offline (file `0x374C9`–
/// `0x37504`).
fn farm(st: &mut State, m: usize) {
    let p = host(st, m);
    if st.word(p + F_ENERGY) == 0 {
        st.set_byte(m + M_FLAGS, st.byte(m + M_FLAGS) & !0x20);
        return;
    }
    st.set_word(p + F_ENERGY, st.word(p + F_ENERGY) - 1);
    let food = st.word(p + F_FOOD);
    if food < STOCK_MAX {
        let bonus = u16::from(st.byte(G_FARMTECH) != 0) * 0x19
            + u16::from(st.byte(p + F_OWNER) == 4) * 0x1C;
        st.set_word(p + F_FOOD, food + 0xC + bonus);
    }
    dirty(st, 0x4000);
}

/// Solar satellite: `+6` energy, `+7` more for owner 3 (file `0x37506`–
/// `0x3752A`).
fn solar(st: &mut State, m: usize) {
    if st.byte(m + M_TYPE) != TYPE_SOLAR {
        return;
    }
    let p = host(st, m);
    let e = st.word(p + F_ENERGY);
    if e < STOCK_MAX {
        let v = 6 + u16::from(st.byte(p + F_OWNER) == 3) * 7;
        st.set_word(p + F_ENERGY, e + v);
        st.set_word(DIRTY1, st.word(DIRTY1) | 0x2);
    }
}

/// `M_BUILD` countdown → colonise completion (file `0x375A5`–`0x37616`).
fn build(st: &mut State, m: usize, rng: &mut Rng) -> MachOut {
    if st.byte(m + M_FLAGS) & 1 == 0 {
        return MachOut::Ran;
    }
    let t = st.word(m + M_BUILD).wrapping_sub(1);
    st.set_word(m + M_BUILD, t);
    if t != 0 {
        st.set_word(MSG_BUILD, t);
        return MachOut::Ran;
    }
    st.set_byte(m + M_FLAGS, st.byte(m + M_FLAGS) & !1);
    colonise(st, rng);
    MachOut::Colonised
}

/// The queued planet goes live (file `0x375C1`–`0x37616`): kind `0xA`,
/// rolled owner `{1,3,4,5}`, population/food, fixed stocks, dirty `0x8`.
fn colonise(st: &mut State, rng: &mut Rng) {
    let p = usize::from(st.word(QUEUED_REC));
    st.set_word(p + F_KIND, 0xA);
    st.set_byte(p + F_OWNER, roll_owner(rng));
    st.set_word(p + F_POP, rng.range(0x3E8));
    st.set_dword(p + F_CREDITS, 0);
    st.set_word(p + F_FOOD, rng.range(0x4B0) + 0x12C);
    st.set_word(p + F_MINERALS, 0x14);
    st.set_word(p + F_FUEL, 0x96);
    st.set_word(p + F_ENERGY, 0x23);
    dirty(st, 0x8);
}

/// `next16 & 7` rejected while `{0,2,≥6}` → `{1,3,4,5}` (file
/// `0x375CA`–`0x375DF`).
fn roll_owner(rng: &mut Rng) -> u8 {
    loop {
        let o = u8::try_from(rng.next16() & 7).unwrap_or(0);
        if matches!(o, 1 | 3 | 4 | 5) {
            return o;
        }
    }
}
