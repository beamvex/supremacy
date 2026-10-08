//! The `cs:0x742C` machine production/mining/build pass and the
//! `cs:0x4434` fleet defence resolution.

mod common;

use common::gsim::{galaxy, mach, rec, set_mach, ship};
use supremacy::game::{self, Battle, MachOut, Rng, State};

/// Two armed ships with `pow` docked at planet record `p`.
fn dock(st: &mut State, p: usize, n: u16, pow: u16) {
    for i in 0..n {
        let s = ship(i);
        st.set_word(s + game::S_POW, pow);
        st.set_word(s + game::S_LINK, u16::try_from(p).unwrap());
        st.set_byte(s + game::S_FLAGS, 2);
        st.set_byte(s + game::S_CREW, 1);
    }
}

#[test]
fn solar_and_miner_produce() {
    let mut st = galaxy();
    let (p, mut rng) = (rec(&st, 0), Rng::new(1));
    set_mach(&mut st, 0, game::TYPE_SOLAR, 0);
    st.set_word(p + game::F_ENERGY, 100);
    game::machine_tick(&mut st, 0, &mut rng);
    assert_eq!(st.word(p + game::F_ENERGY), 106);
    // core miner, owner 5 host → minerals +2+5, fuel +7+0xF
    set_mach(&mut st, 1, game::TYPE_MINER, 0x20);
    st.set_byte(p + game::F_OWNER, 5);
    game::machine_tick(&mut st, 1, &mut rng);
    assert_eq!(st.word(p + game::F_MINERALS), 7);
    assert_eq!(st.word(p + game::F_FUEL), 0x16);
}

#[test]
fn farm_burns_energy_or_goes_offline() {
    let mut st = galaxy();
    let (p, mut rng) = (rec(&st, 0), Rng::new(1));
    let m = set_mach(&mut st, 2, game::TYPE_FARM, 0x20);
    st.set_word(p + game::F_ENERGY, 100);
    game::machine_tick(&mut st, 2, &mut rng);
    assert_eq!(st.word(p + game::F_ENERGY), 99);
    assert_eq!(st.word(p + game::F_FOOD), 0xC); // owner 0 → no bonus
    st.set_word(p + game::F_ENERGY, 0); // energy exhausted
    game::machine_tick(&mut st, 2, &mut rng);
    assert_eq!(st.byte(m + game::M_FLAGS) & 0x20, 0);
}

#[test]
fn build_countdown_colonises_queued_planet() {
    let mut st = galaxy();
    let mut rng = Rng::new(0x77);
    let m = mach(3);
    st.set_byte(m + game::M_TYPE, 5); // any nonzero type
    st.set_byte(m + game::M_FLAGS, 1);
    st.set_word(m + game::M_BUILD, 1);
    let p = rec(&st, 2);
    st.set_word(game::QUEUED_REC, u16::try_from(p).unwrap());
    assert_eq!(game::machine_tick(&mut st, 3, &mut rng), MachOut::Colonised);
    assert_eq!(st.word(p + game::F_KIND), 0xA);
    assert!(matches!(st.byte(p + game::F_OWNER), 1 | 3 | 4 | 5));
    assert_eq!(st.word(p + game::F_MINERALS), 0x14);
    assert_eq!(st.word(p + game::F_FUEL), 0x96);
    assert_eq!(st.word(p + game::F_ENERGY), 0x23);
    assert_eq!(st.dword(p + game::F_CREDITS), 0);
}

#[test]
fn defence_scan_and_overrun() {
    let mut st = galaxy();
    let p = rec(&st, 1);
    st.set_byte(p + game::F_OWNER, 3);
    st.set_byte(p + game::F_SERIAL, 1);
    st.set_word(p + game::F_TROOPS, 0x7530); // regen caps at 0x3095
    st.set_word(game::UIMODE, 0);
    dock(&mut st, p, 2, 1); // two weak armed ships
    assert_eq!(game::defence_tick(&mut st, 1), Battle::Overrun);
    assert_eq!(st.word(p + game::F_DEFENCE), 0);
    assert_eq!(st.word(p + game::F_KIND), 7); // planet fell
    for i in 0..2u16 {
        assert_eq!(st.word(ship(i) + game::S_POW), 0); // ships destroyed
    }
}

#[test]
fn defence_repelled_damages_ships() {
    let mut st = galaxy();
    let p = rec(&st, 1);
    st.set_byte(p + game::F_OWNER, 3);
    st.set_byte(p + game::F_SERIAL, 1);
    st.set_word(p + game::F_TROOPS, 10); // eff = 1 < any defence
    st.set_word(game::UIMODE, 0);
    dock(&mut st, p, 1, 100);
    match game::defence_tick(&mut st, 1) {
        Battle::Repelled | Battle::Reinforced => {}
        other => panic!("expected Repelled/Reinforced, got {other:?}"),
    }
}
