//! The `minifb` window — presents the [`Screen`][crate::video::Screen]
//! per frame and pumps cursor/buttons/keys back through [`Game::step`].
//!
//! Loop body: read minifb state → [`super::Pump`] → `step` → expand the
//! mode-laid-out VRAM through the palette → `update_with_buffer`.
//! Scancodes are set-1 (arrows/Enter/Esc for the `K`-mode nav path).

use minifb::{Key, MouseButton, MouseMode, Scale, Window, WindowOptions};

use super::Game;

const W: usize = 320;
const H: usize = 200;

/// `minifb` key → DOS set-1 scancode.
const SCAN: &[(Key, u8)] = &[
    (Key::Up, 0x48),
    (Key::Down, 0x50),
    (Key::Left, 0x4B),
    (Key::Right, 0x4D),
    (Key::Enter, 0x1C),
    (Key::Escape, 0x01),
    (Key::Space, 0x39),
    (Key::Backspace, 0x0E),
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
    win.set_target_fps(30);
    let mut buf = vec![0u32; W * H];
    let mut prev: Vec<Key> = Vec::new();
    while win.is_open() {
        input(g, &win, &mut prev);
        g.step();
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
