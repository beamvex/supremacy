//! Host services the event VM and the tick-direct routines call into —
//! the parts of the interpreter that are really the platform/frontend
//! layer (printing, menus, sound, the `cs:0x12xx` dispatch slots).
//!
//! The asm calls them directly; the port records them so the shell can
//! hook real implementations later (GAM-7/GAM-9).

/// One host-side effect, in the order the asm issues it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Call {
    /// `call 0x6B06` — print a `0xFF`-terminated string at `ds:` offset.
    Print(u16),
    /// `call 0x2FE7` — menu/dialogue service.
    Menu,
    /// The modal dialog block inside `cs:0x79C3` (`0x2CA9`/`0x5C1E`/
    /// `0x8DC6`/`0x83AA`/`0xA369`/`0xA10F` loop while `cs:0xE713`).
    Dialog,
    /// `call word [0x1264]` — template dispatch slot (screen refresh).
    Refresh,
    /// `call word [0x1262]` — template dispatch slot (draw op at
    /// `cs:0x4309` site: `ax/bx/cx/dx` args, carried as a packed word).
    Draw(u16, u16),
    /// `call word far [cs:0x127C]` — the sound-driver entry (`cl` args).
    Sound(u8),
    /// `call 0x8A5B`/`0x8A3B`/`0x8A80` — sound-channel helpers.
    Chan(u16, u8),
    /// The `0x41`-iteration `[0x924D]` flash loop in `cs:0x7A45`.
    Flash,
    /// `call word [0x125E]` with `dx` — template slot (palette flush).
    Present(u16),
    /// `call 0x5C97` — redraw the selected planet panel.
    PlanetPanel,
    /// `call word [0x1268]` — template slot (blit sprite) with `bx/dx`.
    Blit(u16, u16),
    /// A near call the port doesn't model yet (`cs:` target).
    Native(u16),
    /// `call 0x2CA9` — UI housekeeping used by several op blocks.
    Ui(u16),
    /// `call 0x6709` — the `0xFF`-terminated print interpreter at a
    /// `ds:` string offset with `bx`/`dx` cursor args.
    Text(u16, u16, u16),
    /// `call 0x686C`/`0x68E0` — print a `u16`/`u32` number at `bx`/`dx`.
    Num(u32, u16, u16),
    /// `call word [0x1262]` — the `ax`/`bx`/`cx`/`dx` box op at
    /// `cs:0x4309` (window frame before a popup).
    Rect(u16, u16, u16, u16),
    /// `jmp 0x8B17`/`0x8C65` — the `0xFA` arm's full-screen sequences
    /// (`ds` swap + `call far [cs:0x1278]` image load).
    Screen(u16),
}

/// Host callback — one method keeps call sites uniform; the return word
/// feeds the few services that answer in `ax`/`bl` (defaults `0`).
pub trait VmHost {
    /// Perform one host-side effect; return the `ax`/`bl` answer word.
    fn svc(&mut self, call: Call) -> u16;
}

/// A sink that drops every call — used when no frontend is attached.
pub struct NullHost;

impl VmHost for NullHost {
    fn svc(&mut self, _call: Call) -> u16 {
        0
    }
}
