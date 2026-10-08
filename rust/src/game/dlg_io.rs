//! Save/load dialog actions — `cs:0x2E94` (load), `cs:0x2ED2` (save),
//! the `cs:0x2F2D` filename input, the `0x8E66`/`0x8E75` slot-mode
//! helpers, and the `[0x91DA]` flag setters the confirm menu targets
//! (file `0x32E94`–`0x32F9A`).
//!
//! The filename buffer is `ds:0..0x20`: the input routine sanitises
//! `0x20` → NUL (terminate) and `0x5B` ('[') → `0x5C` ('\\'). The
//! `[0x91E5]` switch after a successful load `jmp`s to the four
//! screen continuations `0x685A`/`0x6847`/`0x6834`/`0x6821`.

use super::cells::{
    DLG_FLAG, LINE_MODE, LOAD_TO, NAME_LEN, NAME_MAX, SLOT_MODE, S_ERROR, S_LEGEND, S_LOADED,
    S_PROMPT, S_SAVED,
};
use super::dialog::dlg_text;
use super::vhost::Call;
use super::vops::Ctx;
use super::{linein, text};

/// `u16` casts for the `ds:` slot constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// Filename buffer extent (file `0x32F49`/`0x32F7D`).
const NAME_SPAN: u16 = 0x20;

/// Post-load continuation — the `cs:` `jmp` target `[0x91E5]` picks
/// (file `0x32EB1`–`0x32ECF`).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Loaded {
    /// `[0x91E5] == 0` → `cs:0x685A`.
    A,
    /// `[0x91E5] == 1` → `cs:0x6847`.
    B,
    /// `[0x91E5] == 2` → `cs:0x6834`.
    C,
    /// Other → `cs:0x6821`.
    D,
}

/// `cs:0x2E94` — the load action (file `0x32E94`–`0x32ED1`): prompt +
/// filename input, `0x8E66` pre-load, `0x83CE` state load (carry →
/// `0x7929` error print and `None`), `0x8E75` post-load, panel +
/// `0x7C50` print, then the `[0x91E5]` switch.
pub fn load_game(c: &mut Ctx) -> Option<Loaded> {
    dlg_text(c, a!(S_PROMPT));
    input_name(c);
    slot_mode(c, 1);
    let name = filename(c);
    if !c.st.load(c.files, &name) {
        err(c);
        return None;
    }
    slot_mode(c, 2);
    c.host.svc(Call::PlanetPanel);
    dlg_text(c, a!(S_LOADED));
    Some(match c.vm.r16(c.st, a!(LOAD_TO)) {
        0 => Loaded::A,
        1 => Loaded::B,
        2 => Loaded::C,
        _ => Loaded::D,
    })
}

/// `cs:0x2ED2` — the save action (file `0x32ED2`–`0x32EE6`): prompt +
/// input, `0x83F3` state save (carry → `0x7929`), else the `0x7C2E`
/// print. `false` when the save failed.
pub fn save_game(c: &mut Ctx) -> bool {
    dlg_text(c, a!(S_PROMPT));
    input_name(c);
    let name = filename(c);
    if !c.st.save(c.files, &name) {
        err(c);
        return false;
    }
    dlg_text(c, a!(S_SAVED));
    true
}

/// The ASCIIZ filename the `ds:0` buffer holds after `input_name`.
fn filename(c: &mut Ctx) -> String {
    let n = (0..NAME_SPAN)
        .position(|i| c.vm.r8(c.st, i) == 0)
        .unwrap_or(usize::from(NAME_SPAN));
    (0..NAME_SPAN)
        .take(n)
        .map(|i| char::from(c.vm.r8(c.st, i)))
        .collect()
}

/// `cs:0x2EE7` — the error print arm (file `0x32EE7`–`0x32EED`).
fn err(c: &mut Ctx) {
    dlg_text(c, a!(S_ERROR));
}

/// `cs:0x8E66`/`0x8E75` — bracket a `[0x1278]` image load with
/// `[0x9CCA]` = 1/2 (file `0x38E66`/`0x38E75`).
fn slot_mode(c: &mut Ctx, m: u8) {
    c.vm.w8(c.st, a!(SLOT_MODE), m);
    c.host.svc(Call::Slot(0x1278, 0));
    c.vm.w8(c.st, a!(SLOT_MODE), 0);
}

/// `cs:0x2F2D` — filename input (file `0x32F2D`–`0x32F90`): reset the
/// `0x4FCB`/`0x4FCC` cells, legend print, `0xA3BC` section, line-input
/// mode `1` around `0xE66E`, input install, then the `ds:0..0x20`
/// sanitiser.
pub fn input_name(c: &mut Ctx) {
    c.vm.w8(c.st, a!(NAME_LEN), 0);
    c.vm.w8(c.st, a!(NAME_MAX), 0x1E);
    dlg_text(c, a!(S_LEGEND));
    input_capture(c);
    c.vm.w8(c.st, a!(NAME_MAX), 9);
    sanitize(c);
}

/// The capture half (file `0x32F43`–`0x32F5E`) — `0xA3BC` swaps the
/// int-33 handler to the buttons-only mask (`0x1E`, `ax=0x14`),
/// line-input mode `1` around the `0xE66E` editor, then `0xA36A`
/// reinstalls the full mask (`0x1F`). `si/bx/bp` = `0`/`0x2B`/`0x20`.
fn input_capture(c: &mut Ctx) {
    c.host.svc(Call::Mouse(0x14, 0x1E, 0));
    c.vm.w8(c.st, a!(LINE_MODE), 1);
    linein::line_input(c, 0, 0x2B, 0x20);
    c.vm.w8(c.st, a!(LINE_MODE), 0);
    c.host.svc(Call::Mouse(0x14, 0x1F, 0));
}

/// The `0x2F64`–`0x2F90` sanitiser — `0x20` → NUL terminates, `0x5B`
/// → `0x5C` remaps, over `ds:0..0x20`.
fn sanitize(c: &mut Ctx) {
    for si in 0..NAME_SPAN {
        match c.vm.r8(c.st, si) {
            0x20 => c.vm.w8(c.st, si, 0),
            0x5B => c.vm.w8(c.st, si, 0x5C),
            _ => {}
        }
    }
}

/// `cs:0x2F01` — confirm "yes": `[0x91DA] = 1` then the `0x78B8`
/// reprint (file `0x32F01`).
pub fn act_yes(c: &mut Ctx) {
    c.vm.w8(c.st, a!(DLG_FLAG), 1);
    text::print_str(c, a!(super::cells::S_CNF_T), 0x2A, 0xB1);
}

/// `cs:0x2F08` — confirm "no": `[0x91DA] = 2` (file `0x32F08`).
pub fn act_no(c: &mut Ctx) {
    c.vm.w8(c.st, a!(DLG_FLAG), 2);
}

/// `cs:0x2F95` — cancel: `[0x91DA] = 0xFF` (file `0x32F95`).
pub fn act_cancel(c: &mut Ctx) {
    c.vm.w8(c.st, a!(DLG_FLAG), 0xFF);
}

/// `cs:0x2F91` — the heap-staging call (`0x844B`) used by the save
/// side: snapshot the `ds:0x7D39..0x9CC4` block into the `0x3829`
/// heap segment (file `0x32F91`, `0x3844B`).
pub fn act_stage(c: &mut Ctx) {
    c.vm.stage_save(c.st);
}
