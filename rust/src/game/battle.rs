use super::consts::{
    DIRTY0, DIRTY1, FLEET_BASE, FLEET_COUNT, FLEET_STRIDE, LINK_REC, MACH_BASE, MACH_COUNT,
    MACH_STRIDE, MSG_REC, REINF_REC, SEL_REC, UIMODE,
};
use super::mach::{M_FLAGS, M_HOST, M_TYPE};
use super::rec::F_SERIAL;
use super::rec::{F_ATTACK, F_DEFENCE, F_KIND, F_TIMER_A, F_TIMER_B, F_TIMER_C, F_TROOPS};
use super::ship::{S_CREW, S_FLAGS, S_LINK, S_POW};
use super::State;

/// Crew ceiling for the reinforcement bonus (file `0x348E9`).
const CREW_MAX: u8 = 0x5D;

/// What a planet's defence pass resolved.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Battle {
    /// No ships inbound or ownerless record.
    Quiet,
    /// Defence held: ships took proportional losses, troops bled
    /// `def·2/100` (file `0x34796`–`0x34973`).
    Repelled,
    /// Defence held with zero troops: surviving crews gained `+7`
    /// (file `0x3489D`–`0x3496F`).
    Reinforced,
    /// Defence broken: linked ships and bound machines destroyed, the
    /// planet becomes kind `7` (file `0x3463F`–`0x34795`).
    Overrun,
}

/// Attacker losses — `pow = pow·pct/100` per linked armed ship; a `0`
/// roll unlinks the ship (file `0x34796`–`0x34973`).
pub(super) fn repelled(st: &mut State, r: usize, eff: u32) -> Battle {
    let def = u32::from(st.word(r + F_DEFENCE));
    let pct = 100 * eff / def;
    let mut sum = 0u16;
    if pct != 0 {
        for i in 0..FLEET_COUNT {
            let s = FLEET_BASE + usize::from(i) * FLEET_STRIDE;
            sum = sum.wrapping_add(damage(st, s, r, pct));
        }
    }
    st.set_word(r + F_ATTACK, sum);
    let bleed = u16::try_from(u32::from(st.word(r + F_DEFENCE)) * 2 / 100).unwrap_or(0);
    let left = st.word(r + F_TROOPS).wrapping_sub(bleed);
    let under = left > st.word(r + F_TROOPS);
    st.set_word(r + F_TROOPS, left);
    if under {
        return reinforce(st, r);
    }
    Battle::Repelled
}

/// One ship's repelled-loss roll — returns the new `pow` (file
/// `0x347DD`–`0x3485B`).
fn damage(st: &mut State, s: usize, r: usize, pct: u32) -> u16 {
    let linked = st.word(s + S_LINK) == u16::try_from(r).unwrap_or(0);
    if !linked || st.byte(s + S_FLAGS) & 2 == 0 {
        return 0;
    }
    let dmg = u16::try_from(u32::from(st.word(s + S_POW)) * pct / 100).unwrap_or(0);
    if dmg == 0 {
        for f in [S_LINK, S_LINK + 2, S_FLAGS] {
            st.set_word(s + f, 0);
        }
        st.set_word(DIRTY1, st.word(DIRTY1) | 0x8);
        return 0;
    }
    st.set_word(s + S_POW, dmg);
    dmg
}

/// Zero every armed ship docked at record `r` (file `0x345DA`–`0x34639`).
pub(super) fn destroy_ships(st: &mut State, r: usize) {
    let abort = st.byte(UIMODE) == 1 && st.word(LINK_REC) == u16::try_from(FLEET_BASE).unwrap_or(0);
    if abort {
        return;
    }
    for i in 0..FLEET_COUNT {
        let s = FLEET_BASE + usize::from(i) * FLEET_STRIDE;
        let linked = st.word(s + S_LINK) == u16::try_from(r).unwrap_or(0);
        if linked && st.byte(s + S_FLAGS) & 2 != 0 {
            for f in 0..8 {
                st.set_byte(s + f, 0);
            }
        }
    }
}

/// Planet falls (file `0x3463F`–`0x34795`) — clears defence/attack,
/// destroys bound machines and their ships, kind → `7`.
pub(super) fn overrun(st: &mut State, r: usize) -> Battle {
    st.set_word(r + F_DEFENCE, 0);
    st.set_word(r + F_ATTACK, 0);
    if st.word(r + F_KIND) == 7 {
        return Battle::Overrun;
    }
    if !gate(st, r) {
        return Battle::Overrun;
    }
    st.set_word(MSG_REC, u16::try_from(r).unwrap_or(0));
    kill_machines(st, r);
    st.set_word(r + F_KIND, 7);
    st.set_word(DIRTY1, st.word(DIRTY1) | 0x8);
    st.set_word(DIRTY0, st.word(DIRTY0) | 0x18);
    Battle::Overrun
}

/// The capture UI gate — modes `0/5/7` always pass, `4` never, others
/// only for the selected record (file `0x34649`–`0x34677`).
fn gate(st: &State, r: usize) -> bool {
    let mode = st.byte(UIMODE);
    matches!(mode, 0 | 5 | 7) || (mode != 4 && st.word(SEL_REC) == u16::try_from(r).unwrap_or(0))
}

/// Machines bound to the planet's serial are destroyed with their
/// docked ships; matching planet timers clear (file `0x3467E`–`0x3475E`).
fn kill_machines(st: &mut State, r: usize) {
    let serial = st.word(r + F_SERIAL);
    for i in 1..=u32::from(MACH_COUNT) {
        let m = MACH_BASE + usize::try_from(i - 1).unwrap_or(0) * MACH_STRIDE;
        if st.word(m + M_HOST) != serial {
            continue;
        }
        let f = st.byte(m + M_FLAGS);
        if f & 0x10 == 0 && f & 0x4 != 0 {
            continue;
        }
        destroy_ships(st, m);
        st.set_byte(m + M_TYPE, 0);
        clear_timer(st, r, i);
    }
}

/// Clear a planet timer matching machine index `i` (file `0x346F7`–
/// `0x34726`).
fn clear_timer(st: &mut State, r: usize, i: u32) {
    let idx = u8::try_from(i).unwrap_or(0);
    for f in [F_TIMER_A, F_TIMER_B, F_TIMER_C] {
        if st.byte(r + f) == idx {
            st.set_byte(r + f, 0);
            return;
        }
    }
}

/// Zero-troops reinforcement — crews of armed linked ships gain `+7`
/// under `0x5D` (file `0x3489D`–`0x3496F`).
pub(super) fn reinforce(st: &mut State, r: usize) -> Battle {
    st.set_word(r + F_TROOPS, 0);
    if st.word(r + F_KIND) == 0xA {
        return Battle::Reinforced;
    }
    st.set_word(REINF_REC, u16::try_from(r).unwrap_or(0));
    for i in 0..FLEET_COUNT {
        let s = FLEET_BASE + usize::from(i) * FLEET_STRIDE;
        let ok = st.byte(s + S_FLAGS) & 0xA == 2
            && st.word(s + S_LINK) == u16::try_from(r).unwrap_or(0)
            && st.byte(s + S_CREW) < CREW_MAX;
        if ok {
            st.set_byte(s + S_CREW, st.byte(s + S_CREW) + 7);
        }
    }
    st.set_word(DIRTY1, st.word(DIRTY1) | 0x8);
    st.set_word(DIRTY0, st.word(DIRTY0) | 0x18);
    Battle::Reinforced
}
