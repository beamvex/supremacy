//! `cs:0x85B1` — the title attract + `0x87D7` scenario picker, driven
//! through `title_enter`/`title_step` with scripted cursor input.
//!
//! The shipped `ds:0x9662` chain yields scenario indices `1, 2, 3, 0`
//! (SMINE, KRART, RORN, WOTOK) as `0x8926` cycles; `0x8984` commits
//! `[0x91E5]` through the `0x6801` `pop ax; ret` preset tail, which
//! `title_step` turns into the shared `0x1D60` post tail (`[0x9C9A]=1`,
//! the base asset pair, `jmp 0x305B` new game).

mod common;

use std::collections::VecDeque;

use supremacy::game::{self, Call, Ctx, Rng, State, Vm, VmHost};
use supremacy::platform::MemFs;

/// `[0x91E5]` — the picker scenario index.
const SCENARIO: usize = 0x91E5;
/// `[0x9C9F]` — the chain cursor.
const CHAIN_PTR: usize = 0x9C9F;
/// `[0x9209]` — the present-variant selector.
const PICK_VAR: usize = 0x9209;
/// `[0x143]` — the video-mode word gating the MCGA slides.
const MODE_WORD: usize = 0x143;

/// The shipped `ds:0x9662` chain template (file `0x1A302`).
const CHAIN: [u8; 0x3C] = [
    0x04, 0, 0, 0, 0, 0, 0, 0, 0x01, 0, 0, 0, //
    0x03, 0, 0, 0, 0, 0, 0, 0, 0x02, 0, 0, 0, //
    0x01, 0, 0, 0, 0xFF, 0xFF, 0xFF, 0xFF, 0x03, 0, 0, 0, //
    0x02, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
/// The shipped `ds:0x9734` picker list (file `0x1A3D4`) — sel word,
/// cycle hotspot (`0xF676`, `x 0x6A..0xCE` / `y 0xA5..0xB8`) and start
/// (`0xF6D4`, `x 0xE2..0x12A`).
const PICK: [u8; 0x2A] = [
    0, 0, //
    0x6A, 0, 0xA5, 0, 0xCE, 0, 0xB8, 0, 0, 0, 0, 0, 0x76, 0xF6, 0x6B, 0x28, 0xFF, 0xFF, 0xFF,
    0x01, //
    0xE2, 0, 0xA5, 0, 0x2A, 0x01, 0xB8, 0, 0, 0, 0, 0, 0xD4, 0xF6, 0x6B, 0x28, 0xFF, 0xFF, 0, 0xFF,
];

/// Recording host — `clicks` are queued `(x, y)` picker presses
/// released the frame after, `latch` keeps `[0x9CCB]` pending so the
/// attract blocks jump straight to `0x87D7`.
struct Host {
    calls: Vec<Call>,
    cs: Vec<u8>,
    texts: VecDeque<u16>,
    clicks: VecDeque<(u16, u16)>,
    down: bool,
    latch: bool,
}

impl VmHost for Host {
    fn svc(&mut self, call: Call) -> u16 {
        self.calls.push(call);
        match call {
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
        if self.down {
            vm.w16(st, w16(game::BUTTONS).unwrap_or(0), 0);
            self.down = false;
            return;
        }
        if vm.r16(st, w16(game::HOT_LIST).unwrap_or(0)) == 0x9D36 {
            if let Some((x, y)) = self.clicks.pop_front() {
                vm.w16(st, w16(game::CUR_X).unwrap_or(0), x);
                vm.w16(st, w16(game::CUR_Y).unwrap_or(0), y);
                vm.w16(st, w16(game::BUTTONS).unwrap_or(0), 1);
                self.down = true;
                return;
            }
        }
        if self.latch {
            vm.w8(st, w16(game::KEY_PEND).unwrap_or(0), 1);
        }
    }
}

/// `Vm`/`State`/`Rng`/`Host`/`MemFs` bundle for the attract tests.
struct Rig {
    vm: Vm,
    st: State,
    rng: Rng,
    host: Host,
    files: MemFs,
}

impl Rig {
    /// Fresh rig — zeroed `Vm`, tpl structs seeded in `st` like the
    /// shipped image's save block.
    fn new() -> Self {
        let mut st = State::new();
        let at = |a: usize| a - 0x7D39;
        st.block[at(0x9662)..at(0x9662) + CHAIN.len()].copy_from_slice(&CHAIN);
        st.block[at(0x9734)..at(0x9734) + PICK.len()].copy_from_slice(&PICK);
        Self {
            vm: Vm::new(),
            st,
            rng: Rng::new(0x42),
            host: Host {
                calls: Vec::new(),
                cs: vec![0; 0x10000],
                texts: VecDeque::new(),
                clicks: VecDeque::new(),
                down: false,
                latch: false,
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
    /// `title_enter` then one `title_step`; returns `self` for chaining.
    fn boot(&mut self) -> &mut Self {
        game::title_enter(&mut self.ctx());
        game::title_step(&mut self.ctx());
        self
    }
    /// `0xFFFF` at `cs:ofs` — a seq stream that ends on first fetch.
    fn stream_end(&mut self, ofs: usize) {
        self.host.cs[ofs] = 0xFF;
        self.host.cs[ofs + 1] = 0xFF;
    }
    /// Drive `title_step` until `Vm::title` clears (bounded).
    fn run(&mut self) -> usize {
        let mut n = 0;
        while self.vm.title.is_some() && n < 0x4000 {
            game::title_step(&mut self.ctx());
            n += 1;
        }
        n
    }
}

#[test]
fn attract_entry_plays_music_and_slide0() {
    let mut r = Rig::new();
    game::title_enter(&mut r.ctx());
    let c = &r.host.calls;
    // `0x85B1`–`0x8629`: buttons-only hook, the music idiom, refresh,
    // `Image(0)`/`Present(0xD)`, `0x9F6` seq arm.
    assert!(c.contains(&Call::Mouse(0x14, 0x1E, 0)));
    assert!(c.contains(&Call::Chan(0x8A80, 0)));
    assert!(c.contains(&Call::SndReset));
    assert!(c.contains(&Call::Song(0x3D7)));
    assert!(c.contains(&Call::Sound(0)));
    assert!(c.contains(&Call::Image(0)));
    assert!(c.contains(&Call::Present(0xD)));
    assert!(r.vm.title.is_some());
    assert_eq!(r.st.word(CHAIN_PTR), 0x9C62);
    assert_eq!(r.st.word(SCENARIO), 0);
    // The `tpl + 0x600` live copies landed — picker list head record.
    assert_eq!(r.vm.r16(&r.st, 0x9D36), 0x006A);
    assert_eq!(r.vm.r16(&r.st, 0x9D4A), 0x00E2);
    assert_eq!(r.vm.r16(&r.st, 0x9D42), 0xF676);
    assert_eq!(r.vm.r16(&r.st, 0x9D56), 0xF6D4);
}

#[test]
fn pending_key_enters_picker() {
    let mut r = Rig::new();
    r.host.latch = true;
    r.boot();
    // One frame of seq poll sees `[0x9CCB]` → `jmp 0x87D7` install.
    let c = &r.host.calls;
    assert!(c.contains(&Call::Mouse(0, 0, 0)));
    assert!(c.contains(&Call::Mouse(0xC, 1, 0x1143)));
    assert!(c.contains(&Call::Mouse(8, 0, 0xBC)));
    assert!(c.contains(&Call::Mouse(7, 0, 0x260)));
    assert!(c.contains(&Call::Image(2)));
    assert!(c.contains(&Call::Present(0xB)));
    assert_eq!(r.st.word(game::HOT_LIST), 0x9D36);
    assert_eq!(r.st.word(game::HOT_COUNT), 2);
}

#[test]
fn non_mcga_skips_to_credits() {
    let mut r = Rig::new();
    r.vm.w16(&mut r.st, u16::try_from(MODE_WORD).unwrap(), 0x0D); // EGA — the 0x8641 test fails
    r.stream_end(0x9F6);
    r.host.latch = true;
    r.boot();
    assert!(!r.host.calls.contains(&Call::Image(5)));
    assert!(r.vm.title.is_some());
}

#[test]
fn mcga_runs_all_three_slides() {
    let mut r = Rig::new();
    r.vm.w16(&mut r.st, u16::try_from(MODE_WORD).unwrap(), 0x13);
    r.stream_end(0x9F6);
    r.stream_end(0xB22);
    r.stream_end(0xC8A);
    r.boot();
    for _ in 0..0x60 {
        game::title_step(&mut r.ctx());
    }
    let c = &r.host.calls;
    assert!(c.contains(&Call::Image(5)));
    assert!(c.contains(&Call::Image(0x3B)));
    assert!(c.contains(&Call::Slot(0x128E, 8)));
}

#[test]
fn credits_loop_repeats_until_input() {
    let mut r = Rig::new();
    r.stream_end(0x9F6); // non-MCGA (`[0x143] = 0`) → slide 0 only
                         // Two scroll pages then the stream-end marker → `0x87D7`.
    r.host.texts.extend([0x0000, 0x0000, 0x0100]);
    r.host.clicks.push_back((0x100, 0xB0));
    r.boot();
    let steps = r.run();
    assert!(r.vm.title.is_none(), "title still live after {steps}");
    let c = &r.host.calls;
    assert!(c.contains(&Call::Text(0x5012)));
    assert!(c.contains(&Call::Text(u16::MAX)));
    assert!(c.contains(&Call::Slot(0x1290, 9)));
    assert!(c.contains(&Call::Slot(0x1292, 0xC)));
    assert!(c.contains(&Call::Native(0x8983)));
}

#[test]
fn cycle_hotspot_walks_the_chain() {
    let mut r = Rig::new();
    r.host.latch = true;
    // Cycle three times (rec0 rect centre 0x9C,0xAE), then start.
    r.host.clicks.extend([(0x9C, 0xAE); 3]);
    r.boot();
    for _ in 0..0x20 {
        game::title_step(&mut r.ctx());
    }
    let c = &r.host.calls;
    // rec0 img 4 scen 1 → rec1 img 3 scen 2 → rec2 img 1 scen 3 var
    // 0xFFFF → `Present(0xA)` + the `0x89B2` stub.
    assert!(c.contains(&Call::Image(4)));
    assert!(c.contains(&Call::Image(3)));
    assert!(c.contains(&Call::Image(1)));
    assert!(c.contains(&Call::Present(0xA)));
    assert!(c.contains(&Call::Native(0x89B2)));
    assert_eq!(r.st.word(SCENARIO), 3);
    assert_eq!(r.st.word(PICK_VAR), 0xFFFF);
    assert_eq!(r.st.word(CHAIN_PTR), 0x9C86);
}

#[test]
fn cycle_wraps_to_the_chain_head() {
    let mut r = Rig::new();
    r.host.latch = true;
    r.host.clicks.extend([(0x9C, 0xAE); 5]);
    r.boot();
    for _ in 0..0x30 {
        game::title_step(&mut r.ctx());
    }
    // 5th cycle: terminator wraps to `ds:0x9C62` → rec0 (scen 1).
    assert_eq!(r.st.word(SCENARIO), 1);
    assert_eq!(r.st.word(CHAIN_PTR), 0x9C6E);
}

#[test]
fn start_picks_wotok_and_restarts() {
    let mut r = Rig::new();
    r.host.latch = true;
    r.host.clicks.push_back((0x100, 0xB0)); // start rect
    r.boot();
    let steps = r.run();
    assert!(r.vm.title.is_none(), "title still live after {steps}");
    // `[0x91E5] = 0` → `0x67E8` WOTOK: base 0x8F66, 8 records, diff 0.
    assert_eq!(r.st.word(game::REC_BASE), 0x8F66);
    assert_eq!(r.st.word(game::REC_TOTAL), 8);
    assert_eq!(r.st.byte(game::DIFFICULTY), 0);
    // The `0x1D60` tail — `[0x9C9A] = 1`, base pair, `jmp 0x305B`:
    // `new_game` snapshot + shell re-enter install.
    assert_eq!(r.st.byte(0x9C9A), 1);
    assert!(r.host.calls.contains(&Call::Load(0x1208)));
    assert!(r.host.calls.contains(&Call::Load(0x1210)));
    assert_eq!(r.st.word(game::HOT_LIST), 0xA0AC);
    // `0x8432` captured the picked config into `stage`.
    let mut snap = State::new();
    snap.block.copy_from_slice(&r.vm.stage[..]);
    assert_eq!(snap.word(game::REC_BASE), 0x8F66);
}

#[test]
fn start_after_full_cycle_picks_rorn() {
    let mut r = Rig::new();
    r.host.latch = true;
    // Cycles leave `[0x91E5] = 3` → `0x6792` RORN + `[0x91E7] = 0xFF`.
    r.host.clicks.extend([(0x9C, 0xAE); 3]);
    r.host.clicks.push_back((0x100, 0xB0));
    r.boot();
    r.run();
    assert_eq!(r.st.word(game::REC_BASE), 0x8486);
    assert_eq!(r.st.byte(0x91E7), 0xFF);
}

#[test]
fn zeroed_chain_is_safe() {
    let mut r = Rig::new();
    r.st.block[0x9662 - 0x7D39..0x9662 - 0x7D39 + CHAIN.len()].fill(0);
    game::dispatch_action(&mut r.ctx(), 0x8926);
    // All-terminator chain bails instead of spinning forever.
    assert!(r.host.calls.contains(&Call::Image(0)));
}
