//! Platform layer: `HostFs` against the shipped asset dir, the PIT chain
//! cadence, and the keyboard ring.

mod common;

use common::game_dir;
use supremacy::platform::{DosFiles, HostFs, Keyboard, Mouse, Timer};

#[test]
fn hostfs_loads_mcg_like_the_loader() {
    let mut fs = HostFs::new(game_dir());
    assert!(fs.find_first("MCG.BIN"));
    let h = fs.open("MCG.BIN").unwrap();
    let mut total = 0;
    let mut chunk = vec![0u8; 0xFFF0];
    loop {
        let n = fs.read(h, &mut chunk);
        total += n;
        if n < chunk.len() {
            break;
        }
    }
    fs.close(h);
    let len = std::fs::metadata(game_dir().join("MCG.BIN")).unwrap().len();
    assert_eq!(total, usize::try_from(len).unwrap());
    assert_eq!(fs.read(h, &mut chunk), 0);
}

#[test]
fn unknown_files_fail_like_dos() {
    let mut fs = HostFs::new(game_dir());
    assert!(!fs.find_first("NOPE.BIN"));
    assert!(fs.open("NOPE.BIN").is_none());
}

#[test]
fn timer_chains_every_third_tick() {
    let mut t = Timer::new();
    assert!(!t.tick() && !t.tick() && t.tick());
    assert!(!t.tick() && !t.tick() && t.tick());
}

#[test]
fn divisor_zero_means_18_hz() {
    let mut t = Timer::new();
    assert_eq!(t.freq_mhz(), 18_206);
    t.program(0xFFFF / 3);
    assert_eq!(t.freq_mhz(), 1_193_182_000 / (0xFFFF / 3));
}

#[test]
fn keyboard_stack_is_lifo() {
    let mut k = Keyboard::new();
    k.irq(0x1C);
    k.irq(0x9C);
    assert_eq!(k.pop(), Some(0x9C));
    assert_eq!(k.pop(), Some(0x1C));
    assert_eq!(k.pop(), None);
    for i in 0u8..20 {
        k.push(i);
    }
    assert_eq!(k.pop(), Some(9));
    assert_eq!((0..9).filter(|_| k.pop().is_some()).count(), 9);
    assert_eq!(k.pop(), None);
}

#[test]
fn keyboard_irq_latches_and_tracks_cad() {
    let mut k = Keyboard::new();
    assert_eq!(k.take_pending(), None);
    k.irq(0x1C);
    assert_eq!(k.take_pending(), Some(0x1C));
    assert_eq!(k.take_pending(), None);
    for s in [0x1D, 0x38, 0x53] {
        k.irq(s);
    }
    assert!(k.reboot);
}

#[test]
fn k_mode_feeds_buttons() {
    use supremacy::platform::MouseEvent;
    let mut m = Mouse::new(true);
    assert_eq!(m.feed_scan(0x1C), Some(MouseEvent::LeftDown));
    assert_eq!(m.buttons, 1);
    assert_eq!(m.feed_scan(0x9C), Some(MouseEvent::Release));
    assert_eq!(m.buttons, 0);
    assert_eq!(m.feed_scan(0x01), Some(MouseEvent::RightDown));
    assert_eq!(m.feed_scan(0x48), None);
}

#[test]
fn driver_mode_feeds_synthetic_codes() {
    use supremacy::platform::MouseEvent;
    let mut m = Mouse::new(false);
    assert_eq!(m.feed_scan(0x7E), Some(MouseEvent::LeftDown));
    assert_eq!(m.feed_scan(0xFD), Some(MouseEvent::Release));
    assert_eq!(m.feed_scan(0x1C), None);
}

#[test]
fn nav_dir_table() {
    use supremacy::platform::{nav_dir, NavDir};
    assert_eq!(nav_dir(0x48, false), Some(NavDir::Up));
    assert_eq!(nav_dir(0x29, false), None);
    assert_eq!(nav_dir(0x29, true), Some(NavDir::Up));
    assert_eq!(nav_dir(0x4E, true), Some(NavDir::Right));
}

#[test]
fn mouse_state_tracks_int33_shape() {
    let mut m = Mouse::new(true);
    m.set(160, 100, 1);
    assert_eq!(
        (m.x, m.y, m.buttons, m.keyboard_emulated),
        (160, 100, 1, true)
    );
}
