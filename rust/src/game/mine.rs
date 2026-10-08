use super::consts::{DIRTY0, G_DEEPMINE, MSG_MINE, SEL_MACH};
use super::mach::{M_DEPOSIT, M_FLAGS, M_HOST, M_LINK, M_OPS, M_TYPE};
use super::State;

/// Dirty-bit helper for `+0x9144`.
fn dirty(st: &mut State, bits: u16) {
    st.set_word(DIRTY0, st.word(DIRTY0) | bits);
}

/// Mining ops (file `0x3752B`–`0x375A4`) — drains `M_DEPOSIT` by
/// `0x19`/`0x32` per op while flag `0x8` and `M_OPS` remain.
pub(super) fn mine(st: &mut State, m: usize) {
    let active = st.byte(m + M_FLAGS) & 8 != 0 && st.word(m + M_OPS) != 0;
    if !active {
        return;
    }
    let trec = 0x9B12 + usize::from(st.byte(m + M_TYPE).wrapping_sub(1)) * 0x30;
    if st.word(trec + 0xC) != 0xFFFF {
        let rate = if st.byte(G_DEEPMINE) != 0 { 0x19 } else { 0x32 };
        let dep = st.word(m + M_DEPOSIT);
        st.set_word(m + M_DEPOSIT, dep - dep.min(rate));
    }
    finish_op(st, m);
}

/// `M_OPS−1` bookkeeping + completion relink (file `0x37563`–`0x375A4`).
fn finish_op(st: &mut State, m: usize) {
    let ops = st.word(m + M_OPS) - 1;
    st.set_word(m + M_OPS, ops);
    let sel = st.word(SEL_MACH) == u16::try_from(m).unwrap_or(0);
    if ops != 0 {
        if sel {
            dirty(st, 0x2000);
        }
        if st.byte(m + M_FLAGS) & 1 != 0 {
            st.set_word(MSG_MINE, ops);
        }
        return;
    }
    if sel {
        dirty(st, 0x3000);
    }
    st.set_byte(m + M_FLAGS, st.byte(m + M_FLAGS) & !8);
    let link = usize::from(st.word(m + M_LINK));
    st.set_byte(m + M_HOST, st.byte(link + 0x28));
    if st.byte(m + M_FLAGS) & 1 != 0 {
        st.set_byte(m + M_FLAGS, 0x11);
    }
}
