//! `0xFA` end-to-end — the `cs:0x8B17`/`cs:0x8C65` ending sequences
//! driven through `tick_step` + `end_step`, including the `0x1D35`
//! restart through the `0x85B1` title attract and scenario picker.
//!
//! Monitor mapping (file `0x382E5`–`0x38338`): faction-record kind `7`
//! (fallen) or no kind-`7` planet → **A = defeat** (`[0x91F1]`, code
//! `2`, `*DTH` assets); record `0` kind `0xA` (inhabited) or no
//! kind-`0xA` planet → **B = victory** (`[0x91F0]`, code `3`, `*WIN`
//! assets). Both `jmp 0x1D35` — the tail asserts on the `*IN` load
//! order, the attract's `Song(0x3D7)` marker and the picker install.

mod common;

use std::collections::VecDeque;

use common::gsim::galaxy;
use supremacy::game::{self, Call, Ctx, Rng, State, Tick, Vm, VmHost};
use supremacy::platform::MemFs;

/// `[0x91E5]` — the scenario index the endings pick pages on.
const SCENARIO: usize = 0x91E5;

/// The shipped `ds:0x9662` scenario-chain template (file `0x1A302`) —
/// four `{img, ·, →[0x9209], ·, →[0x91E5], ·}` records + terminator.
const CHAIN: [u8; 0x3C] = [
    0x04, 0, 0, 0, 0, 0, 0, 0, 0x01, 0, 0, 0, //
    0x03, 0, 0, 0, 0, 0, 0, 0, 0x02, 0, 0, 0, //
    0x01, 0, 0, 0, 0xFF, 0xFF, 0xFF, 0xFF, 0x03, 0, 0, 0, //
    0x02, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
/// The shipped `ds:0x9734` picker-list template (file `0x1A3D4`) — the
/// sel word, the cycle hotspot (`0xF676`) and the start hotspot
/// (`0xF6D4`, rect `x 0xE2..0x12A` / `y 0xA5..0xB8`).
const PICK: [u8; 0x2A] = [
    0, 0, //
    0x6A, 0, 0xA5, 0, 0xCE, 0, 0xB8, 0, 0, 0, 0, 0, 0x76, 0xF6, 0x6B, 0x28, 0xFF, 0xFF, 0xFF,
    0x01, //
    0xE2, 0, 0xA5, 0, 0x2A, 0x01, 0xB8, 0, 0, 0, 0, 0, 0xD4, 0xF6, 0x6B, 0x28, 0xFF, 0xFF, 0, 0xFF,
];

/// Seed the tpl structs into the staging heap — the `0x844B` restore
/// at the ending's input-exit carries them into the save block
/// exactly like the shipped image does.
fn seed_tpl(vm: &mut Vm) {
    let at = |a: usize| a - 0x7D39;
    vm.stage[at(0x9662)..at(0x9662) + CHAIN.len()].copy_from_slice(&CHAIN);
    vm.stage[at(0x9734)..at(0x9734) + PICK.len()].copy_from_slice(&PICK);
}

/// Recording host — `texts` scripts the `menu_text` page results
/// (`0x100` = stream end), `keys` the channel-A stack `Call::KeyPop`
/// drains. `attract` flips on the `0x85B1` entry's `Song(0x3D7)` —
/// `pump_input` then keeps a pending key latched so every attract
/// block exits to `0x87D7` on its first frame, and clicks the start
/// hotspot once the picker list is live.
struct Host {
    calls: Vec<Call>,
    cs: Vec<u8>,
    texts: VecDeque<u16>,
    keys: VecDeque<u8>,
    attract: bool,
    clicked: bool,
}

impl VmHost for Host {
    fn svc(&mut self, call: Call) -> u16 {
        if call == Call::Song(0x3D7) {
            self.attract = true;
        }
        self.calls.push(call);
        match call {
            Call::KeyPop => self.keys.pop_front().map_or(0, u16::from),
            Call::Text(_) => self.texts.pop_front().unwrap_or(0x100),
            _ => 0,
        }
    }
    fn cs_word(&mut self, ofs: u16) -> u16 {
        let a = usize::from(ofs);
        u16::from(self.cs[a]) | u16::from(self.cs[a + 1]) << 8
    }
    fn pump_input(&mut self, vm: &mut Vm, st: &mut State) {
        let w16 = u16::try_from;
        if !self.clicked && vm.r16(st, w16(game::HOT_LIST).unwrap_or(0)) == 0x9D36 {
            vm.w16(st, w16(game::CUR_X).unwrap_or(0), 0x100);
            vm.w16(st, w16(game::CUR_Y).unwrap_or(0), 0xB0);
            vm.w16(st, w16(game::BUTTONS).unwrap_or(0), 1);
            self.clicked = true;
        } else if self.attract {
            vm.w8(st, w16(game::KEY_PEND).unwrap_or(0), 1);
        }
    }
}

/// `Vm`/`State`/`Rng`/`Host`/`MemFs` bundle for the ending tests.
struct Rig {
    vm: Vm,
    st: State,
    rng: Rng,
    host: Host,
    files: MemFs,
}

impl Rig {
    /// Fresh rig — zeroed `Vm`, preset-3 galaxy, empty script, tpl
    /// structs seeded into `stage` for the `0x844B` restore.
    fn new() -> Self {
        let mut vm = Vm::new();
        seed_tpl(&mut vm);
        Self {
            vm,
            st: galaxy(),
            rng: Rng::new(0x42),
            host: Host {
                calls: Vec::new(),
                cs: vec![0; 0x10000],
                texts: VecDeque::new(),
                keys: VecDeque::new(),
                attract: false,
                clicked: false,
            },
            files: MemFs::new(),
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
    /// Tick the `code − 1` → `code` edge through `tick_step`.
    fn seq(&mut self, code: u8) -> Tick {
        self.st.set_byte(game::SEQ, code - 1);
        game::tick_step(&mut self.ctx())
    }
    /// Record `i`'s `+0x0C` kind word.
    fn kind(&mut self, i: u16, k: u16) {
        let r = usize::from(self.st.rec_ofs(i)) + game::F_KIND;
        self.st.set_word(r, k);
    }
    /// `0xFFFF` at `cs:ofs` — a two-op seq stream that ends on the
    /// first wrap (the `0x0A0A`/`0x0DBE` reveal streams).
    fn stream_end(&mut self, ofs: usize) {
        self.host.cs[ofs] = 0xFF;
        self.host.cs[ofs + 1] = 0xFF;
    }
    /// Drive `end_step` until `Vm::end` clears (bounded).
    fn run(&mut self) -> usize {
        let mut n = 0;
        while self.vm.end.is_some() && n < 0x4000 {
            game::end_step(&mut self.ctx());
            n += 1;
        }
        n
    }
    /// Index of `q` in the recorded calls.
    fn pos(&self, q: Call) -> usize {
        self.host
            .calls
            .iter()
            .position(|&c| c == q)
            .unwrap_or(usize::MAX)
    }
}

#[test]
fn defeat_fires_on_fallen_faction() {
    let mut r = Rig::new();
    r.kind(r.st.planets(), 7); // the faction record, not rec0
    assert_eq!(r.seq(0xFA), Tick::Endgame(Some(game::Ending::A)));
    assert_eq!(r.st.byte(0x91F1), 0xFF);
    assert_eq!(r.st.byte(0x9C9A), 2);
    // `0x8322` arm: buttons-only hook, `*DTH.BIN`/`*DTH.GPH`, reinstall.
    assert_eq!(
        r.host.calls[..4],
        [
            Call::Mouse(0x14, 0x1E, 0),
            Call::Load(0x122C),
            Call::Load(0x1237),
            Call::Mouse(0x14, 0x1F, 0)
        ]
    );
    assert_eq!(
        r.host.calls[4..8],
        [
            Call::Mouse(0x14, 0x1E, 0),
            Call::Chan(0x8A80, 2),
            Call::SndReset,
            Call::Song(0x95A)
        ]
    );
}

#[test]
fn defeat_sequence_runs_to_restart() {
    let mut r = Rig::new();
    r.kind(r.st.planets(), 7);
    r.st.set_word(SCENARIO, 0); // WOTOK → the 0x5712 page
    r.stream_end(0xA0A);
    r.host.keys.push_back(0x1C);
    r.seq(0xFA);
    let steps = r.run();
    assert!(r.vm.end.is_none(), "still running after {steps} frames");
    let c = &r.host.calls;
    assert!(c.contains(&Call::Text(0x5712)));
    assert!(c.contains(&Call::Text(0x58F2)));
    assert!(c.contains(&Call::Present(0xE)));
    assert!(c.contains(&Call::Image(0)));
    assert!(c.contains(&Call::KeyPop));
    // `0x1D35` — `*IN` pair before the attract's `0x85B1` entry marker
    // (`Song(0x3D7)`), the base pair after the picker hands off.
    let title = r.pos(Call::Song(0x3D7));
    assert!(r.pos(Call::Load(0x1218)) < title);
    assert!(r.pos(Call::Load(0x1222)) < title);
    assert!(r.pos(Call::Load(0x1208)) > title);
    assert!(c.contains(&Call::Load(0x1210)));
    // The `0x87D7` picker install and its `Image(2)`/`Present(0xB)` show.
    assert!(c.contains(&Call::Image(2)));
    assert!(c.contains(&Call::Present(0xB)));
    // The start hotspot `0x8984` → preset → `jmp 0x305B` new game:
    // the shell list is reinstalled (`[0x9194] = 0xA0AC`).
    assert_eq!(
        r.vm.r16(&r.st, u16::try_from(game::HOT_LIST).unwrap_or(0)),
        0xA0AC
    );
}

#[test]
fn victory_fires_on_record0() {
    let mut r = Rig::new();
    r.kind(0, 0xA); // home record inhabited → B
    assert_eq!(r.seq(0xFA), Tick::Endgame(Some(game::Ending::B)));
    assert_eq!(r.st.byte(0x91F0), 0xFF);
    assert_eq!(r.st.byte(0x9C9A), 3);
    assert_eq!(
        r.host.calls[..4],
        [
            Call::Mouse(0x14, 0x1E, 0),
            Call::Load(0x1242),
            Call::Load(0x124D),
            Call::Mouse(0x14, 0x1F, 0)
        ]
    );
    assert_eq!(
        r.host.calls[4..8],
        [
            Call::Mouse(0x14, 0x1E, 0),
            Call::Chan(0x8A80, 1),
            Call::SndReset,
            Call::Song(0x5B7)
        ]
    );
}

#[test]
fn victory_pages_credits_then_restart() {
    let mut r = Rig::new();
    r.kind(0, 0xA);
    r.st.set_word(SCENARIO, 0);
    // Two scroll pages, then the stream-end page.
    r.host.texts.extend([0x0000, 0x0100]);
    r.host.keys.push_back(0x1C);
    r.seq(0xFA);
    r.run();
    let c = &r.host.calls;
    assert!(c.contains(&Call::Text(0x5953)));
    assert!(c.contains(&Call::Text(u16::MAX)));
    assert!(c.contains(&Call::Text(0x58F2)));
    assert!(c.contains(&Call::Slot(0x1292, 0xC)));
    // Non-RORN skips the 0xDBE reveal (`Present(0xF)`/`Image(0)`).
    assert!(!c.contains(&Call::Present(0xF)));
    assert!(c.contains(&Call::Song(0x3D7)));
    assert!(r.vm.end.is_none());
}

#[test]
fn victory_rorn_shows_final_image() {
    let mut r = Rig::new();
    r.kind(0, 0xA);
    r.st.set_word(SCENARIO, 3); // RORN → the 0x5E52 page + 0xDBE image
    r.stream_end(0xDBE);
    r.host.texts.extend([0x0100]);
    r.host.keys.push_back(0x1C);
    r.seq(0xFA);
    r.run();
    let c = &r.host.calls;
    assert!(c.contains(&Call::Text(0x5E52)));
    assert!(c.contains(&Call::Present(0xF)));
    assert!(c.contains(&Call::Image(0)));
    assert!(r.vm.end.is_none());
}
