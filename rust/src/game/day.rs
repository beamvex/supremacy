use super::consts::{DAY, TICK};
use super::State;

/// Day rollover — `cs:0x6A7B` (file `0x36A7B`–`0x36AD1`): the
/// tick-in-day counter `[0x91BA]` runs `1..=0x40` and wraps to `1`,
/// bumping the day counter `[0x91BC]`; the rest of the asm redraws the
/// date readout, which is the shell's job.
pub fn day_tick(st: &mut State) {
    let mut t = st.word(TICK) + 1;
    if t == 0x41 {
        t = 1;
        st.set_word(DAY, st.word(DAY) + 1);
    }
    st.set_word(TICK, t);
}
