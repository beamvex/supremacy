//! The `cs:0x73D9` sequencer/dispatcher — machine range, planet range,
//! defence range, `0xFF` day rollover — plus `day_tick` itself.

mod common;

use common::gsim::{galaxy, step};
use supremacy::game::{self, Rng, Tick};

#[test]
fn sequencer_runs_each_arm() {
    let mut st = galaxy();
    let mut rng = Rng::new(0x42);
    st.set_word(game::SEQ, 0);
    for _ in 1..0x20u8 {
        step(&mut st, &mut rng, 'm'); // codes 1..0x20 → machines
    }
    for _ in 0x20..0x38u16 {
        step(&mut st, &mut rng, '?');
    }
    step(&mut st, &mut rng, 'p'); // 0x38 → planet 0
    let n = st.word(game::REC_TOTAL);
    st.set_byte(game::SEQ, u8::try_from(0x38 + n - 1).unwrap());
    step(&mut st, &mut rng, 'd'); // 0x38+n → defence
    st.set_byte(game::SEQ, 0xFE);
    st.set_word(game::TICK, 0x40);
    assert_eq!(step(&mut st, &mut rng, 'y'), Tick::Day);
    assert_eq!(st.word(game::TICK), 1); // 0x40+1 = 0x41 → wrap
    assert_eq!(st.word(game::DAY), 1);
}

#[test]
fn day_rollover_counts() {
    let mut st = galaxy();
    st.set_word(game::TICK, 5);
    game::day_tick(&mut st);
    assert_eq!(st.word(game::TICK), 6);
    assert_eq!(st.word(game::DAY), 0);
    st.set_word(game::TICK, 0x40);
    game::day_tick(&mut st);
    assert_eq!(st.word(game::TICK), 1);
    assert_eq!(st.word(game::DAY), 1);
}
