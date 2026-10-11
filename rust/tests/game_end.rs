//! `0xFA` end-to-end — the `cs:0x8B17`/`cs:0x8C65` ending sequences
//! driven through `tick_step` + `end_step`.
//!
//! Monitor mapping (file `0x382E5`–`0x38338`): faction-record kind `7`
//! (fallen) or no kind-`7` planet → **A = defeat** (`[0x91F1]`, code
//! `2`, `*DTH` assets); record `0` kind `0xA` (inhabited) or no
//! kind-`0xA` planet → **B = victory** (`[0x91F0]`, code `3`, `*WIN`
//! assets). Both `jmp 0x1D35` — the title-attract + new-game restart
//! the tail asserts on (`Native(0x85B1)` then the `*IN`/base loads).

mod common;

use std::collections::VecDeque;

use common::gsim::galaxy;
use supremacy::game::{self, Call, Ctx, Rng, State, Tick, Vm, VmHost};
use supremacy::platform::MemFs;

/// `[0x91E5]` — the scenario index the endings pick pages on.
const SCENARIO: usize = 0x91E5;

/// Recording host — `texts` scripts the `menu_text` page results
/// (`0x100` = stream end), `keys` the channel-A stack `Call::KeyPop`
/// drains; `pump_input` latches `[0x9CCB]` once the `0x85B1` attract
/// marker is out — the `Ph::Title` exit the asm takes on input.
struct Host {
    calls: Vec<Call>,
    cs: Vec<u8>,
    texts: VecDeque<u16>,
    keys: VecDeque<u8>,
}

impl VmHost for Host {
    fn svc(&mut self, call: Call) -> u16 {
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
        if self.calls.contains(&Call::Native(0x85B1)) {
            vm.w8(st, u16::try_from(game::KEY_PEND).unwrap_or(0), 1);
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
    /// Fresh rig — zeroed `Vm`, preset-3 galaxy, empty script.
    fn new() -> Self {
        Self {
            vm: Vm::new(),
            st: galaxy(),
            rng: Rng::new(0x42),
            host: Host {
                calls: Vec::new(),
                cs: vec![0; 0x10000],
                texts: VecDeque::new(),
                keys: VecDeque::new(),
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
    assert!(c.contains(&Call::Native(0x85B1)));
    // `0x1D35` — `*IN` pair before the attract, base pair after.
    assert!(r.pos(Call::Load(0x1218)) < r.pos(Call::Native(0x85B1)));
    assert!(r.pos(Call::Load(0x1222)) < r.pos(Call::Native(0x85B1)));
    assert!(r.pos(Call::Load(0x1208)) > r.pos(Call::Native(0x85B1)));
    assert!(c.contains(&Call::Load(0x1210)));
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
    assert!(c.contains(&Call::Native(0x85B1)));
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
