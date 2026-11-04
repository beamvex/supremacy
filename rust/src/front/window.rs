//! The `minifb` window — presents the [`Screen`][crate::video::Screen]
//! per vsync and pumps cursor/buttons/keys back through [`Game::step`].
//!
//! All the work happens in the [`super::Host::set_vsync`] hook —
//! `pump_input` runs it at every `0x2CA9` boundary (each `Game::step`
//! head and inside every `vsync`'d wait/frame loop), so the nested
//! screen `frame_loop`s and dialogs that never return to this
//! function's outer loop still present their framebuffer and receive
//! input. Scancodes are set-1 — [`SCAN`] covers the `ds:0x7CBD`
//! keymap (the save-name editor's alnum/punct/keypad set) plus the
//! `K`-mode nav arrows, Enter and Esc.

use std::cell::RefCell;
use std::rc::Rc;

use minifb::{Key, MouseButton, MouseMode, Scale, Window, WindowOptions};

#[cfg(feature = "audio")]
use super::audio::Out;
use super::Game;
use crate::palette::Palette;
use crate::video::Screen;

const W: usize = 320;
const H: usize = 200;
/// Game frame rate — also the audio render stride (`rate/30` samples
/// per `step`).
const FPS: u32 = 30;

use super::scan::SCAN;

/// Raw hardware-level button source `(left, right)` — the frontend
/// or-s it over minifb's event-fed `get_mouse_down` (see [`buttons`]).
pub type ButtonProbe = Box<dyn FnMut() -> (bool, bool)>;

/// Run `g` until the window closes. `probe` supplies a raw button
/// level [`buttons`] or-s over the event-fed state.
///
/// # Errors
/// Propagates `minifb` window-creation/update failures.
pub fn run(g: &mut Game, probe: Option<ButtonProbe>) -> Result<(), minifb::Error> {
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
    let win = Rc::new(RefCell::new(win));
    install_vsync(g, Rc::clone(&win), probe);
    #[cfg(feature = "audio")]
    let mut pcm: Vec<f32> = Vec::new();
    while win.borrow().is_open() {
        g.step();
        #[cfg(feature = "audio")]
        if let Some(o) = &audio {
            pcm.resize(o.rate as usize / usize::try_from(FPS).unwrap_or(30), 0.0);
            g.host.snd.render(&mut pcm);
            o.push(&pcm);
        }
    }
    g.host.set_vsync(None);
    Ok(())
}

/// Install the live vsync hook on `g.host` — `pump_input` runs it at
/// every `0x2CA9` boundary (each `Game::step` head and inside every
/// `vsync`'d wait/frame-loop), so both the frame present and fresh
/// input land mid-step like the asm's int-9/int-33 ISRs + retrace.
/// `update_with_buffer` pumps the event queue, presents and
/// rate-limits in one call; the input read that follows sees the
/// just-pumped state. A closed window exits the process — the nested
/// game loops never return here to check `is_open` themselves.
fn install_vsync(g: &mut Game, win: Rc<RefCell<Window>>, probe: Option<ButtonProbe>) {
    let mut probe = probe;
    let mut prev: Vec<Key> = Vec::new();
    let mut buf = vec![0u32; W * H];
    g.host.set_vsync(Some(Box::new(move |host| {
        let mut w = win.borrow_mut();
        blit(&mut buf, &host.scr, &host.pal);
        if w.update_with_buffer(&buf, W, H).is_err() || !w.is_open() {
            std::process::exit(0);
        }
        input(&mut host.pump, &w, &mut prev, &mut probe);
    })));
}

/// Poll minifb → pump — cursor (already buffer-space), button edges,
/// make/break scancodes.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn input(
    pump: &mut super::Pump,
    win: &Window,
    prev: &mut Vec<Key>,
    probe: &mut Option<ButtonProbe>,
) {
    let (mx, my) = win.get_mouse_pos(MouseMode::Clamp).unwrap_or_default();
    let (l, r) = buttons(win, probe);
    pump.move_to(mx.max(0.) as u16, my.max(0.) as u16);
    pump.set_buttons(l, r);
    keys(pump, win, prev);
}

/// Button level — minifb's event-fed state or-ed with the raw
/// [`ButtonProbe`] level while the cursor is inside the window.
/// `mouseDown:` never reaches the view when AppKit swallows the click
/// (a press on a non-key window dispatches nothing), and the int-33h
/// driver modelled here reads the buttons unconditionally — the
/// hardware level is the faithful source. Outside the window the
/// probe is ignored so presses on other windows can't leak in.
fn buttons(win: &Window, probe: &mut Option<ButtonProbe>) -> (bool, bool) {
    let ev = (
        win.get_mouse_down(MouseButton::Left),
        win.get_mouse_down(MouseButton::Right),
    );
    match probe.as_mut().map(|p| p()) {
        Some((cl, cr)) if win.get_mouse_pos(MouseMode::Discard).is_some() => {
            (ev.0 || cl, ev.1 || cr)
        }
        _ => ev,
    }
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
fn blit(buf: &mut [u32], scr: &Screen, pal: &Palette) {
    for (i, &p) in scr.pixels().iter().enumerate() {
        let [r, gr, b] = pal.rgb(usize::from(p));
        buf[i] = u32::from(r) << 16 | u32::from(gr) << 8 | u32::from(b);
    }
}
