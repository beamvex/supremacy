//! The `cs:0x3D1E` planet update — food consumption, coverage tracking,
//! tax income, and the `net·pop/400+1` population model.

mod common;

use common::gsim::{galaxy, rec};
use supremacy::game::{self, State};

/// Planet `i` as a colonised record with the given food/pop/tax/owner.
fn colonised(st: &mut State, i: u16, pop: u16, food: u16) -> usize {
    let r = rec(st, i);
    st.set_word(r + game::F_KIND, 0xA);
    st.set_word(r + game::F_POP, pop);
    st.set_word(r + game::F_FOOD, food);
    r
}

#[test]
fn consumption_coverage_and_income() {
    let mut st = galaxy();
    let r = colonised(&mut st, 1, 2400, 100);
    st.set_byte(r + game::F_TAX, 40);
    st.set_byte(r + game::F_OWNER, 2);
    st.set_word(game::TICK, 2); // even — consumption, no income
    assert!(!game::sim_planet(&mut st, 1));
    assert_eq!(st.word(r + game::F_FOOD), 100 - 2400 / 0xF0);
    assert_eq!(st.byte(r + game::F_COVER), 1); // tracks 100-40 up by 1
                                               // odd tick — income = pop*tax/125 (owner 2); the even tick's
                                               // net-negative growth score already shrank pop, so read it back.
    st.set_word(game::TICK, 3);
    let pop = u32::from(st.word(r + game::F_POP));
    game::sim_planet(&mut st, 1);
    assert_eq!(st.dword(r + game::F_CREDITS), pop * 40 / 125);
}

#[test]
fn starvation_warns_every_16th_tick() {
    let mut st = galaxy();
    let r = colonised(&mut st, 1, 0xF000, 3); // < need (0xF000/0xF0 = 0x100)
    st.set_word(game::TICK, 4);
    assert!(!game::sim_planet(&mut st, 1)); // no warn at tick 4
    assert_eq!(st.word(r + game::F_FOOD), 0);
    st.set_word(r + game::F_FOOD, 3);
    st.set_word(game::TICK, 16);
    assert!(game::sim_planet(&mut st, 1)); // warn at tick 16
}

#[test]
fn population_grows_and_shrinks() {
    let mut st = galaxy();
    let r = colonised(&mut st, 1, 2000, 60000);
    st.set_byte(r + game::F_COVER, 99); // growth 33, decline ~0
    st.set_word(game::TICK, 2);
    game::sim_planet(&mut st, 1);
    let grown = st.word(r + game::F_POP);
    assert!(grown > 2000, "pop {grown} should grow at full coverage");
    st.set_byte(r + game::F_TAX, 100);
    for _ in 0..64 {
        st.set_byte(r + game::F_COVER, 0);
        game::sim_planet(&mut st, 1);
    }
    assert!(st.word(r + game::F_POP) < grown, "starved+taxed pop falls");
}
