//! GAM-29 — the C-compiled `AdLib`/Roland driver (`audio::Ar`, file
//! `0x2900`): the jump-table far exports (`init`/`service`/`load_song`/
//! `select`/`reset`/`fade`), the `ds:si` song record copy + `map`/`tbl`
//! queue base, the `AdLib` OPL2 `apply`/`service` stream and the Roland
//! MPU-401 UART stream.

mod common;

use common::rec::Rec;
use common::{read, root};
use supremacy::args::Sound;
use supremacy::audio::Ar;
use supremacy::front::audio::{Audio, Drv};

fn img() -> Vec<u8> {
    read(&root().join("decompiled/GAME.unpacked.exe"))
}

/// `Call::Song(0x03D7)` — the title song record (`ds = 0x03D7` →
/// file `0x03D7·16 + 0xC00` = `0x4970`, the embedded `song0`).
fn song0(img: &[u8]) -> &[u8] {
    &img[0x4970..]
}

/// Detect replies for the canonical OPL2 timer test — `st0` quiet,
/// `st1 = 0xC0` (the 327th status read, per `audio_dev.rs`).
fn opl_present() -> std::collections::VecDeque<u8> {
    let mut r = vec![0u8; 327];
    r[326] = 0xC0;
    r.into()
}

#[test]
fn adlib_init_detects_and_installs_the_timer() {
    let mut a = Ar::new(Rec::new(), img().as_slice());
    a.ports.replies = opl_present();
    assert_eq!(a.init(0), 1); // `ax = [0x155] − 3` = 0 → AdLib
    let outs = a.ports.outs();
    // `0x2952` install — PIT ch0 programmed `0x5D37`.
    assert!(outs.contains(&(0x43, 0x36)));
    assert!(outs.contains(&(0x40, 0x37)));
    assert!(outs.contains(&(0x40, 0x5D)));
    // `0x2D55` `opl_init` — register-init writes on `0x388`/`0x389`.
    assert!(outs.iter().any(|&(p, _)| p == 0x388));
    assert!(outs.iter().any(|&(p, _)| p == 0x389));
}

#[test]
fn roland_init_handshakes_and_sets_up_uart() {
    let mut a = Ar::new(Rec::new(), img().as_slice());
    // `reset` ACK + the `0x3F` UART-mode command ACK.
    a.ports.replies.extend([0x00, 0x00, 0xFE, 0x00, 0x00, 0xFE]);
    assert_eq!(a.init(1), 1); // `ax` low byte nonzero → Roland
    let outs = a.ports.outs();
    assert!(outs.contains(&(0x331, 0xFF))); // MPU reset
    assert!(outs.contains(&(0x331, 0x3F))); // UART-mode enter
                                            // `0x281D` — B0/E0 init triplets + the `cs:0x1C07` setup SysEx.
    assert!(outs.contains(&(0x330, 0xF0)));
    assert!(outs.iter().any(|&(_, v)| v & 0xF0 == 0xB0));
}

#[test]
fn absent_adlib_init_fails_and_stays_idle() {
    let mut a = Ar::new(Rec::new(), img().as_slice());
    // `last = 0` — status never reports the timer flags → absent.
    assert_eq!(a.init(0), 0);
    a.ports.ops.clear();
    a.load_song(song0(&img()));
    a.select(0);
    for _ in 0..20 {
        a.service();
    }
    assert!(a.ports.ops.is_empty()); // `mode = 0xFF` gates everything
}

#[test]
fn load_song_select_then_service_writes_opl() {
    let mut a = Ar::new(Rec::new(), img().as_slice());
    a.ports.replies = opl_present();
    assert_eq!(a.init(0), 1);
    a.load_song(song0(&img()));
    a.select(0); // export 4 — the queue cursor + channel setup
    a.ports.ops.clear();
    for _ in 0..60 {
        a.service(); // export 2 — the int-8 fetch/apply heartbeat
    }
    let outs = a.ports.outs();
    // `apply_a`/`note_on_a` — A0/B0 frequency+key pairs on `0x389`.
    let regs: Vec<u8> = outs
        .iter()
        .filter(|&&(p, _)| p == 0x388)
        .map(|&(_, v)| v)
        .collect();
    assert!(regs.iter().any(|&r| r & 0xF0 == 0xA0));
    assert!(regs.iter().any(|&r| r & 0xF0 == 0xB0));
}

#[test]
fn roland_load_song_sends_timbre_sysexes() {
    let i = img();
    let mut a = Ar::new(Rec::new(), i.as_slice());
    a.ports.replies.extend([0x00, 0x00, 0xFE, 0x00, 0x00, 0xFE]);
    assert_eq!(a.init(1), 1);
    a.ports.ops.clear();
    a.load_song(song0(&i));
    let midi: Vec<u8> = a
        .ports
        .outs()
        .iter()
        .filter(|&&(p, _)| p == 0x330)
        .map(|&(_, v)| v)
        .collect();
    // `0x2104` — timbre uploads (SysEx `F0` blocks) + the `C1`
    // program sweep for each `progmap` slot.
    assert!(midi.contains(&0xF0));
    let c1 = midi.iter().fold(0usize, |n, &v| n + usize::from(v == 0xC1));
    assert_eq!(c1, 0x20);
    // Each uploaded `timbre` is six sends: the `0x1C6F` name block, the
    // `0x1C61` index block and four `0x3A` param blocks (`0x1C1D`) —
    // `F0` heads, `F7` tails. (The `0x281D` setup `SysEx` went at
    // `init`, already cleared above.)
    let s = song0(&i);
    let qi = qbase_idx(s) * 2;
    let qbase = usize::from(u16::from_le_bytes([s[0x118 + qi], s[0x119 + qi]])) * 16;
    let uploads = (0..0x20usize)
        .filter(|&n| {
            s[0x98 + n * 2] != 0 && s[0x99 + n * 2] != 0 && s[qbase + 0x118 + n * 0x100] != 0x20
        })
        .count();
    let f7 = midi.iter().fold(0usize, |n, &v| n + usize::from(v == 0xF7));
    assert_eq!(f7, uploads * 6);
}

/// `tbl[maxmap + 1]` — the `0x236A` scan index `load_song` uses for
/// `qbase` (highest `map[0..0x7F]` value + 1).
fn qbase_idx(s: &[u8]) -> usize {
    let max = s[0x18..0x97].iter().copied().max().unwrap_or(0);
    usize::from(max) + 1
}

#[test]
fn reset_keys_off_and_gates_service() {
    let mut a = Ar::new(Rec::new(), img().as_slice());
    a.ports.replies = opl_present();
    assert_eq!(a.init(0), 1);
    a.load_song(song0(&img()));
    a.select(0);
    a.ports.ops.clear();
    a.reset(); // export 5 — `en = 0` + all-notes-off
    let outs = a.ports.outs();
    // `0x205B` — the `0x1270` note-off tails: B0-register writes with
    // the key bit clear.
    let b0: Vec<u8> = outs
        .iter()
        .filter(|&&(p, _)| p == 0x388)
        .map(|&(_, v)| v)
        .filter(|&r| r & 0xF0 == 0xB0)
        .collect();
    assert!(!b0.is_empty());
    a.ports.ops.clear();
    for _ in 0..10 {
        a.service();
    }
    assert!(a.ports.ops.is_empty()); // `en = 0` gates the fetch
}

#[test]
fn fade_runs_down_and_stops() {
    let mut a = Ar::new(Rec::new(), img().as_slice());
    a.ports.replies = opl_present();
    assert_eq!(a.init(0), 1);
    a.load_song(song0(&img()));
    a.select(0);
    a.fade_set(0x40); // export 6 — arm the `0x1F05` fader
    assert!(a.fade_query()); // export 7 — `[0x2060]` armed
    for _ in 0..300 {
        a.service();
    }
    assert!(!a.fade_query()); // `mvol` hit 0 → `en = 0`, `fade = 0`
    a.ports.ops.clear();
    for _ in 0..10 {
        a.service();
    }
    assert!(a.ports.ops.is_empty());
}

#[test]
fn engine_runs_the_adlib_driver() {
    let i = img();
    let mut a = Audio::new(Sound::AdLib, i.as_slice(), 44_100);
    assert!(matches!(a.drv, Drv::Ar(_)));
    a.song(0x03D7, i.as_slice()); // `Call::Song` — `[cs:0x1278]`
    a.sound(0); // `Call::Sound` — `[cs:0x127C]` select
    let mut pcm = vec![0f32; 44_100];
    a.render(&mut pcm);
    assert!(pcm.iter().any(|&s| s != 0.0));
}

#[test]
fn engine_runs_the_roland_driver() {
    let i = img();
    let mut a = Audio::new(Sound::Roland, i.as_slice(), 44_100);
    assert!(matches!(a.drv, Drv::Ar(_)));
    a.song(0x03D7, i.as_slice());
    a.sound(0);
    let mut pcm = vec![0f32; 22_050];
    a.render(&mut pcm); // MPU bytes are accepted and dropped — silence
    assert_eq!(a.dropped, 0); // the exports route, nothing is dropped
}
