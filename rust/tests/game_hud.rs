//! The `0x20..0x38`/`0xA7`/`0xAA`/`0xF9`/`0xFA`/`0xFC`/`0xFD` arms —
//! ship crew ageing, HUD reprints, sprite marking, the orders popup and
//! the endgame monitor.

mod common;

use common::gsim::galaxy;
use supremacy::game::{self, Call, Ctx, Rng, ShipOut, State, Tick, Vm, VmHost};

struct Rec(Vec<Call>);
impl VmHost for Rec {
    fn svc(&mut self, call: Call) -> u16 {
        self.0.push(call);
        0
    }
}

struct Rig {
    vm: Vm,
    st: State,
    rng: Rng,
    host: Rec,
    files: supremacy::platform::MemFs,
}

impl Rig {
    fn new() -> Self {
        Self {
            vm: Vm::new(),
            st: galaxy(),
            rng: Rng::new(0x42),
            host: Rec(Vec::new()),
            files: supremacy::platform::MemFs::new(),
        }
    }
    fn ctx(&mut self) -> Ctx<'_> {
        Ctx {
            vm: &mut self.vm,
            st: &mut self.st,
            rng: &mut self.rng,
            host: &mut self.host,
            files: &mut self.files,
        }
    }
    fn seq(&mut self, code: u8) -> Tick {
        self.st.set_byte(game::SEQ, code - 1);
        game::tick_step(&mut self.ctx())
    }
    fn kind(&mut self, i: u16, k: u16) {
        let r = usize::from(self.st.rec_ofs(i)) + game::F_KIND;
        self.st.set_word(r, k);
    }
}

#[test]
fn crew_ages_by_difficulty() {
    let mut r = Rig::new();
    let ship = game::FLEET_BASE;
    for (d, want) in [(0u8, 3u8), (1, 2), (2, 1)] {
        r.st.set_byte(game::DIFFICULTY, d);
        r.st.set_byte(ship + game::S_CREW, 0);
        assert_eq!(game::ship_tick(&mut r.ctx(), 0), ShipOut::Aged);
        assert_eq!(r.st.byte(ship + game::S_CREW), want);
    }
    r.st.set_byte(ship + game::S_CREW, 0x63);
    game::ship_tick(&mut r.ctx(), 0);
    assert_eq!(r.st.byte(ship + game::S_CREW), 0x64); // clamp
    r.st.set_byte(ship + game::S_FLAGS, 2);
    assert_eq!(game::ship_tick(&mut r.ctx(), 0), ShipOut::Skip);
}

#[test]
fn selected_ship_gets_panel() {
    let mut r = Rig::new();
    let ship = u16::try_from(game::FLEET_BASE).unwrap();
    r.st.set_byte(game::UIMODE, 1);
    r.st.set_word(game::LINK_REC, ship);
    r.st.set_byte(game::DIFFICULTY, 2);
    r.st.set_word(game::FLEET_BASE + game::S_POW, 1); // crew resets when 0
    r.st.set_byte(game::FLEET_BASE + game::S_CREW, 0x20);
    let row = usize::from(0x7438u16 + 3 * 0x13);
    r.vm.low[row] = b'O';
    r.vm.low[row + 1] = 0xFF;
    assert_eq!(game::ship_tick(&mut r.ctx(), 0), ShipOut::Panel);
    assert_eq!(r.st.byte(0x91D6), 9 - 0x21 / 16); // crew 0x21 → rating
    assert_eq!(r.st.byte(0x91DE), 0xFF);
    assert_eq!(
        r.host.0,
        [
            // num_pad(0x21) → "   33" across five cells
            Call::Glyph(0x20, 0x46, 0xC0),
            Call::Glyph(0x20, 0x47, 0xC0),
            Call::Glyph(0x20, 0x48, 0xC0),
            Call::Glyph(b'3', 0x49, 0xC0),
            Call::Glyph(b'3', 0x4A, 0xC0),
            Call::Glyph(b'O', 0x3B, 0x3F)
        ]
    );
}

#[test]
fn hud_arms_gate_on_uimode() {
    let mut r = Rig::new();
    assert_eq!(r.seq(0xFD), Tick::HudCredits); // uimode 0 → no print
    assert!(r.host.0.is_empty());
    r.st.set_byte(game::UIMODE, 1);
    let frac = usize::from(r.st.rec_ofs(r.st.planets()));
    r.st.set_dword(frac + game::F_CREDITS, 0x1_2345);
    r.st.set_word(frac + game::F_POP, 0x1234);
    assert_eq!(r.seq(0xFD), Tick::HudCredits);
    assert_eq!(r.seq(0xFC), Tick::HudPop);
    let mut want = vec![];
    for (i, ch) in "   74565".bytes().enumerate() {
        want.push(Call::Glyph(ch, 0x45 + i as u16, 0x32));
    }
    for (i, ch) in " 4660".bytes().enumerate() {
        want.push(Call::Glyph(ch, 0x49 + i as u16, 0x0B));
    }
    assert_eq!(r.host.0, want);
}

#[test]
fn mark_sprites_sets_pending() {
    let mut r = Rig::new();
    for (i, v) in [1u8, 2, 4].iter().enumerate() {
        r.vm.low[0x7562 + i] = *v;
    }
    r.vm.low[0x7565] = 0xFF;
    assert_eq!(r.seq(0xAA), Tick::Sprites);
    assert_eq!(&r.vm.low[0x7562..0x7566], &[0x81, 0x82, 0x84, 0xFF]);
}

#[test]
fn endgame_fires_on_branch_conditions() {
    let mut r = Rig::new();
    r.kind(0, 1); // rec0 not colonised — skip the B early-out
    for i in 1..r.st.planets() {
        r.kind(i, 7);
    }
    assert_eq!(r.seq(0xFA), Tick::Endgame(Some(game::Ending::B)));
    assert_eq!(r.st.byte(0x91F0), 0xFF);
    assert_eq!(r.st.byte(0x9C9A), 3);
    assert_eq!(r.host.0, [Call::Screen(0x8C65)]);
}

#[test]
fn endgame_quiet_while_contested() {
    let mut r = Rig::new();
    r.kind(0, 7);
    r.kind(1, 0xA);
    assert_eq!(r.seq(0xFA), Tick::Endgame(None));
    assert_eq!(r.st.byte(0x91F0), 0);
    assert!(r.host.0.is_empty());
}

#[test]
fn orders_popup_copies_entry() {
    let mut r = Rig::new();
    r.st.set_byte(game::UIMODE, 7);
    r.st.set_byte(0x91F4, 1);
    let sel = r.st.rec_ofs(0);
    r.st.set_word(game::SEL_REC, sel);
    r.st.set_word(usize::from(sel) + 0x2E, 1); // order pending → pick 1
    let e: u16 = 0x4EBA + 0x1E;
    for i in 0..27u16 {
        r.vm.low[usize::from(e + i)] = u8::try_from(i + 1).unwrap();
    }
    assert_eq!(r.seq(0xF9), Tick::Orders(true));
    assert_eq!(r.st.word(0x9198), 0x0201);
    assert_eq!(r.st.word(0x9255), 0x0403);
    assert_eq!(r.st.byte(0x91D6), 27);
    assert_eq!(r.st.byte(0x91F4), 0);
    assert_eq!(r.host.0[0], Call::Rect(0x1C, 0x6, 0x3, 0x41));
}
