//! The `minifb` window — presents the [`Screen`][crate::video::Screen]
//! per frame and pumps cursor/buttons/keys back through [`Game::step`].
//!
//! Loop body: read minifb state → [`super::Pump`] → `step` → expand the
//! mode-laid-out VRAM through the palette → `update_with_buffer`.
//! Scancodes are set-1 — [`SCAN`] covers the `ds:0x7CBD` keymap (the
//! save-name editor's alnum/punct/keypad set) plus the `K`-mode nav
//! arrows, Enter and Esc.

use minifb::{Key, MouseButton, MouseMode, Scale, Window, WindowOptions};

#[cfg(feature = "audio")]
use super::audio::Out;
use super::Game;

const W: usize = 320;
const H: usize = 200;
/// Game frame rate — also the audio render stride (`rate/30` samples
/// per `step`).
const FPS: u32 = 30;

/// `minifb` key → DOS set-1 scancode — every code the game's
/// `ds:0x7CBD` keymap and the K-mode nav path consume (arrows, Enter,
/// Esc, the line editor's alnum/punct/keypad set).
const SCAN: &[(Key, u8)] = &[
    (Key::Escape, 0x01),
    (Key::Key1, 0x02),
    (Key::Key2, 0x03),
    (Key::Key3, 0x04),
    (Key::Key4, 0x05),
    (Key::Key5, 0x06),
    (Key::Key6, 0x07),
    (Key::Key7, 0x08),
    (Key::Key8, 0x09),
    (Key::Key9, 0x0A),
    (Key::Key0, 0x0B),
    (Key::Minus, 0x0C),
    (Key::Equal, 0x0D),
    (Key::Backspace, 0x0E),
    (Key::Q, 0x10),
    (Key::W, 0x11),
    (Key::E, 0x12),
    (Key::R, 0x13),
    (Key::T, 0x14),
    (Key::Y, 0x15),
    (Key::U, 0x16),
    (Key::I, 0x17),
    (Key::O, 0x18),
    (Key::P, 0x19),
    (Key::Enter, 0x1C),
    (Key::NumPadEnter, 0x1C),
    (Key::A, 0x1E),
    (Key::S, 0x1F),
    (Key::D, 0x20),
    (Key::F, 0x21),
    (Key::G, 0x22),
    (Key::H, 0x23),
    (Key::J, 0x24),
    (Key::K, 0x25),
    (Key::L, 0x26),
    (Key::Semicolon, 0x27),
    (Key::Backslash, 0x2B),
    (Key::Z, 0x2C),
    (Key::X, 0x2D),
    (Key::C, 0x2E),
    (Key::V, 0x2F),
    (Key::B, 0x30),
    (Key::N, 0x31),
    (Key::M, 0x32),
    (Key::Comma, 0x33),
    (Key::Period, 0x34),
    (Key::Slash, 0x35),
    (Key::Space, 0x39),
    (Key::Up, 0x48),
    (Key::Down, 0x50),
    (Key::Left, 0x4B),
    (Key::Right, 0x4D),
    (Key::Home, 0x47),
    (Key::NumPad7, 0x47),
    (Key::NumPad8, 0x48),
    (Key::PageUp, 0x49),
    (Key::NumPad9, 0x49),
    (Key::NumPad4, 0x4B),
    (Key::NumPad5, 0x4C),
    (Key::NumPad6, 0x4D),
    (Key::End, 0x4F),
    (Key::NumPad1, 0x4F),
    (Key::NumPad2, 0x50),
    (Key::PageDown, 0x51),
    (Key::NumPad3, 0x51),
    (Key::Insert, 0x52),
    (Key::NumPad0, 0x52),
    (Key::Delete, 0x53),
    (Key::NumPadDot, 0x53),
];

/// Run `g` until the window closes.
///
/// # Errors
/// Propagates `minifb` window-creation/update failures.
pub fn run(g: &mut Game) -> Result<(), minifb::Error> {
    let mut win = Window::new(
        "Supremacy",
        W,
        H,
        WindowOptions {
            scale: Scale::X4,
            ..WindowOptions::default()
        },
    )?;
    win.set_target_fps(usize::try_from(FPS).unwrap_or(30));
    #[cfg(feature = "audio")]
    let audio = Out::open();
    #[cfg(feature = "audio")]
    if let Some(o) = &audio {
        g.host.snd.set_rate(o.rate);
    }
    let mut buf = vec![0u32; W * H];
    let mut prev: Vec<Key> = Vec::new();
    #[cfg(feature = "audio")]
    let mut pcm: Vec<f32> = Vec::new();
    while win.is_open() {
        input(g, &win, &mut prev);
        g.step();
        #[cfg(feature = "audio")]
        if let Some(o) = &audio {
            pcm.resize(o.rate as usize / usize::try_from(FPS).unwrap_or(30), 0.0);
            g.host.snd.render(&mut pcm);
            o.push(&pcm);
        }
        blit(&mut buf, g);
        win.update_with_buffer(&buf, W, H)?;
    }
    Ok(())
}

/// Poll minifb → pump — cursor (already buffer-space), button edges,
/// make/break scancodes.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn input(g: &mut Game, win: &Window, prev: &mut Vec<Key>) {
    let (mx, my) = win.get_mouse_pos(MouseMode::Clamp).unwrap_or_default();
    g.host.pump.move_to(mx.max(0.) as u16, my.max(0.) as u16);
    g.host.pump.set_buttons(
        win.get_mouse_down(MouseButton::Left),
        win.get_mouse_down(MouseButton::Right),
    );
    keys(&mut g.host.pump, win, prev);
}

/// Make/break scancodes for the mapped keys that changed since last
/// frame.
fn keys(pump: &mut super::Pump, win: &Window, prev: &mut Vec<Key>) {
    let now = win.get_keys();
    for (k, scan) in SCAN {
        match (prev.contains(k), now.contains(k)) {
            (false, true) => pump.key(*scan),
            (true, false) => pump.key(scan | 0x80),
            _ => {}
        }
    }
    *prev = now;
}

/// Expand VRAM → packed `0xRRGGBB` through the palette.
fn blit(buf: &mut [u32], g: &Game) {
    for (i, &p) in g.host.scr.pixels().iter().enumerate() {
        let [r, gr, b] = g.host.pal.rgb(usize::from(p));
        buf[i] = u32::from(r) << 16 | u32::from(gr) << 8 | u32::from(b);
    }
}
