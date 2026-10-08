use super::consts::{DIRTY0, G_FAMINE, G_KILL_AUX, G_KILL_ON, G_KILL_REC, TICK};
use super::rec::{F_COVER, F_CREDITS, F_DECLINE, F_FOOD, F_GROWTH, F_KIND, F_OWNER, F_POP, F_TAX};
use super::State;

/// Food drain divisor — `pop / 0xF0` per tick (file `0x33D5F`).
const FOOD_NEED: u16 = 0xF0;
/// Population ceiling (file `0x33EED`).
const POP_MAX: u16 = 0x7530;
/// Net-growth → population divisor (file `0x33E9A`).
const POP_RATE: u32 = 0x190;

/// One planet's simulation tick — `cs:0x3D1E` (file `0x33D1E`–`0x33F17`).
///
/// Only `F_KIND == 0xA` (inhabited) records simulate. Returns `true` when
/// the starvation warning fired — the asm splices the planet name into
/// `ds:0x8305` and prints `ds:0x82C0` every 16th tick; display is the
/// shell's job.
pub fn sim_planet(st: &mut State, idx: u16) -> bool {
    let r = usize::from(st.rec_ofs(idx));
    if st.word(r + F_KIND) != 0xA {
        return false;
    }
    let warn = consume(st, r);
    coverage(st, r);
    if st.word(TICK) & 1 == 1 {
        income(st, r);
        return warn;
    }
    scores(st, r);
    population(st, r);
    warn
}

/// `or` into dirty word `[0x9144]`.
fn dirty(st: &mut State, bits: u16) {
    st.set_word(DIRTY0, st.word(DIRTY0) | bits);
}

/// Food consumption (file `0x33D47`–`0x33DAA`) — `true` when the
/// every-16th-tick starvation message would print.
fn consume(st: &mut State, r: usize) -> bool {
    let food = st.word(r + F_FOOD);
    if food == 0 {
        st.set_byte(r + F_COVER, 0);
        dirty(st, 0x40);
        return false;
    }
    let need = st.word(r + F_POP) / FOOD_NEED;
    if food < need {
        st.set_byte(r + F_COVER, 0);
    }
    st.set_word(r + F_FOOD, food - food.min(need));
    dirty(st, 0x4000);
    food < need && st.word(TICK).trailing_zeros() >= 4
}

/// Coverage gauge — `[+0x1F]` tracks `100 − tax` by ±1 (file `0x33DB0`).
fn coverage(st: &mut State, r: usize) {
    let target = 100u8.saturating_sub(st.byte(r + F_TAX));
    let cur = st.byte(r + F_COVER);
    match cur.cmp(&target) {
        std::cmp::Ordering::Less => st.set_byte(r + F_COVER, cur + 1),
        std::cmp::Ordering::Greater => st.set_byte(r + F_COVER, cur - 1),
        std::cmp::Ordering::Equal => return,
    }
    dirty(st, 0xC0);
}

/// Tax income on odd ticks (file `0x33DCA`–`0x33DFD`) —
/// `pop·tax / (owner ∈ {1,2} ? 125 : 200)` into the credits dword.
fn income(st: &mut State, r: usize) {
    let pop = u32::from(st.word(r + F_POP));
    let tax = u32::from(st.byte(r + F_TAX));
    let div = if matches!(st.byte(r + F_OWNER), 1 | 2) {
        0x7D
    } else {
        0xC8
    };
    let credits = st.dword(r + F_CREDITS) + pop * tax / div;
    st.set_dword(r + F_CREDITS, credits);
    dirty(st, 0x4);
}

/// Growth/decline scores on even ticks (file `0x33E00`–`0x33E6C`).
fn scores(st: &mut State, r: usize) {
    let cover = st.byte(r + F_COVER);
    let growth = if st.byte(G_FAMINE) != 0 {
        cover >> 1
    } else {
        cover / 3
    };
    st.set_byte(r + F_GROWTH, growth);
    let decline = (100 - cover) / 4 + st.byte(r + F_TAX) / 4;
    st.set_byte(r + F_DECLINE, decline);
    dirty(st, 0x40);
}

/// Population update (file `0x33E74`–`0x33F17`) — `net·pop/400 + 1`
/// growth/decline, `0x7530` cap, then the delayed-kill check.
fn population(st: &mut State, r: usize) {
    let net = i16::from(st.byte(r + F_GROWTH)) - i16::from(st.byte(r + F_DECLINE));
    let pop = st.word(r + F_POP);
    match net.cmp(&0) {
        std::cmp::Ordering::Greater if pop < 2 => return,
        std::cmp::Ordering::Greater => grow(st, r, net, pop),
        std::cmp::Ordering::Less => shrink(st, r, net, pop),
        std::cmp::Ordering::Equal => {
            delayed_kill(st, r);
            return;
        }
    }
    dirty(st, 0x20);
    let p = st.word(r + F_POP);
    st.set_word(r + F_POP, p.min(POP_MAX));
    delayed_kill(st, r);
}

/// Growth arm — `net·pop/400 + 1` (file `0x33E8D`–`0x33EB7`).
fn grow(st: &mut State, r: usize, net: i16, pop: u16) {
    let g = u32::from(net.cast_unsigned()) * u32::from(pop) / POP_RATE + 1;
    st.set_word(r + F_POP, pop.wrapping_add(u16::try_from(g).unwrap_or(0)));
}

/// Decline arm — same magnitude, clamped to `pop` (file `0x33EBE`–
/// `0x33F0B`).
fn shrink(st: &mut State, r: usize, net: i16, pop: u16) {
    let loss =
        (u32::from((-net).cast_unsigned()) * u32::from(pop) / POP_RATE + 1).min(u32::from(pop));
    st.set_word(r + F_POP, pop - u16::try_from(loss).unwrap_or(0));
}

/// `[0x821F]`/`[0x8220]` delayed depopulation (file `0x33EF9`–`0x33F14`).
fn delayed_kill(st: &mut State, r: usize) {
    let armed = st.byte(G_KILL_ON) != 0
        && st.word(r + F_POP) != 0
        && st.word(G_KILL_REC) == u16::try_from(r).unwrap_or(0);
    if armed {
        st.set_word(r + F_POP, 0);
        st.set_byte(G_KILL_AUX, 0);
    }
}
