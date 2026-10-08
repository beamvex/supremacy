use super::battle::{overrun, reinforce, repelled, Battle};
use super::consts::{DIRTY0, DIRTY1, FLEET_BASE, FLEET_COUNT, FLEET_STRIDE, SCAN_REC, SEL_REC};
use super::rec::{F_29, F_ATTACK, F_DEFENCE, F_OWNER, F_TROOPS};
use super::ship::{S_CREW, S_FLAGS, S_GUNS, S_LINK, S_LOAD, S_POW};
use super::State;

/// Crew-weight divisor in the defence formula (file `0x344C9`).
const CREW_DIV: u32 = 0x48;
/// Troop regeneration cap (file `0x34571`).
const TROOPS_MAX: u16 = 0x3095;

/// One planet's defence pass — `cs:0x4434` (file `0x34434`–`0x34973`).
pub fn defence_tick(st: &mut State, idx: u16) -> Battle {
    let r = usize::from(st.rec_ofs(idx));
    st.set_word(SCAN_REC, u16::try_from(r).unwrap_or(0));
    if st.byte(r + F_OWNER) == 0 {
        return Battle::Quiet;
    }
    fleet_scan(st, r);
    if st.word(r + F_TROOPS) == 0 {
        if st.word(r + F_DEFENCE) == 0 {
            return Battle::Quiet;
        }
        return reinforce(st, r);
    }
    regen(st, r);
    resolve(st, r)
}

/// Σ ship strength → `F_ATTACK`, weighted → `F_DEFENCE` (file
/// `0x34465`–`0x3453A`).
fn fleet_scan(st: &mut State, r: usize) {
    let (mut raw, mut def) = (0u16, 0u16);
    for i in 0..FLEET_COUNT {
        let s = FLEET_BASE + usize::from(i) * FLEET_STRIDE;
        let linked = st.word(s + S_LINK) == u16::try_from(r).unwrap_or(0);
        if linked && st.byte(s + S_FLAGS) & 2 != 0 {
            let (pow, crew) = (st.word(s + S_POW), u32::from(st.byte(s + S_CREW)));
            raw = raw.wrapping_add(pow);
            // `mul`/`div` keep `ax` — both stages truncate to 16 bits.
            let w = (u32::from(pow) * crew / CREW_DIV) & 0xFFFF;
            let guns = u32::from(st.word(s + S_GUNS)) + u32::from(st.word(s + S_LOAD));
            let wt = guns + u32::from(st.byte(r + F_29)) + 1;
            let add = u16::try_from((w * wt) & 0xFFFF).unwrap_or(0);
            def = def.wrapping_add(pow).wrapping_add(add);
        }
    }
    st.set_word(r + F_DEFENCE, def);
    st.set_word(r + F_ATTACK, raw);
    if st.word(SEL_REC) == u16::try_from(r).unwrap_or(0) {
        st.set_word(DIRTY1, st.word(DIRTY1) | 0x8);
        st.set_word(DIRTY0, st.word(DIRTY0) | 0x10);
    }
}

/// Troop regeneration — owner `4`→`+4`, `5`→`+3`, else `+1`; record `0`
/// skips (file `0x34553`–`0x34578`).
fn regen(st: &mut State, r: usize) {
    if usize::from(st.rec_ofs(0)) == r {
        return;
    }
    let rate = match st.byte(r + F_OWNER) {
        4 => 4,
        5 => 3,
        _ => 1,
    };
    let t = st.word(r + F_TROOPS).saturating_add(rate);
    st.set_word(r + F_TROOPS, t.min(TROOPS_MAX));
}

/// The eff-vs-defence comparison (file `0x3457D`–`0x345DA`).
fn resolve(st: &mut State, r: usize) -> Battle {
    let (troops, def) = (st.word(r + F_TROOPS), st.word(r + F_DEFENCE));
    if def == 0 {
        return overrun(st, r);
    }
    let eff = (u32::from(troops) * (2 + u32::from(st.byte(r + F_29))) / 100).max(1);
    if u32::from(def) > eff {
        return repelled(st, r, eff);
    }
    super::battle::destroy_ships(st, r);
    overrun(st, r)
}
