//! Host services the event VM and the tick-direct routines call into —
//! the parts of the interpreter that are really the platform/frontend
//! layer (printing, menus, sound, the `cs:0x12xx` dispatch slots).
//!
//! The asm calls them directly; the port records them so the shell can
//! hook real implementations later (GAM-7/GAM-9).

use super::state::State;
use super::vm::Vm;

/// One host-side effect, in the order the asm issues it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Call {
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
    /// `call word [0x125E]` with `dx` — template slot (palette
    /// present): row `dx` of the `ds:0x4394` table into DAC `1..=31`
    /// (`dx == 6` = the full `ds:0x4994` master), then `pal_adj` —
    /// the routine also reads the `ds:[0x9CA1]` shade cell, so hosts
    /// dispatch it through [`VmHost::svc_st`].
    Present(u16),
    /// `call 0x5C97` — redraw the selected planet panel.
    PlanetPanel,
    /// `call word [0x1268]` — template slot (blit sprite) with `bx/dx`.
    Blit(u16, u16),
    /// `call/jmp word near [0x1268]` — the char-put slot where `ax`
    /// carries the glyph/attr byte (`0x6A8E` prints `0x2F`, `0x6AD2`
    /// prints `0x3E`, the `0x6B8F` ticker prints its stream).
    Glyph(u8, u16, u16),
    /// A near call the port doesn't model yet (`cs:` target).
    Native(u16),
    /// `call 0x2CA9` — UI housekeeping used by several op blocks.
    Ui(u16),
    /// `call word [0x1262]` — the `ax`/`bx`/`cx`/`dx` box op at
    /// `cs:0x4309` (window frame before a popup).
    Rect(u16, u16, u16, u16),
    /// The int-21 asset loaders — `ax=0x4E00` find-first on the
    /// filename cell at `ds:arg` (the `ds:0x1208`+ `???.???` table the
    /// mode select patches with the set stem), then open/read/close
    /// into the load heap, retrying through the `0x8AC0` insert-disk
    /// prompt on failure (`0x20F2`/`0x208E`/`0x21BF`/`0x215B`/`0x2228`/
    /// `0x228C`/`0x22F5`/`0x2359`).
    Load(u16),
    /// `call word far [cs:0x1278]` — the `load_song` far export;
    /// `ds:0` holds the song record, the arg is the segment the asm
    /// loads into `ds` (`0x095A` defeat / `0x05B7` victory / `0x03D7`
    /// title / `0x0C3E` game).
    Song(u16),
    /// `call word far [cs:0x1280]` — the sound-driver reset far export.
    SndReset,
    /// `call word [0x1296]` — `menu_text` (`cs:0x2F629`): render one
    /// page of the `ss:` stream cursor `cs:0xF672`. The arg stores a
    /// fresh cursor first unless it is `u16::MAX` (the resume calls).
    /// Returns `ax` = `end << 8 | cs:0xFB15` — `end` is the `0x00`-op
    /// stream terminator the asm latches into `[0x9CCB]` itself (the
    /// caller mirrors it); the low byte is the page's second header
    /// byte the scroll variants test.
    Text(u16),
    /// `call 0x8891`'s starfield body — the `cs:0x4EC4` table walk
    /// (move, respawn out-of-bounds, `[0x1294]` mark) for `cx = arg`
    /// stars (`[0x9206]`). The `0x2CA9` head is the caller's `vsync`;
    /// the `[0x9206]` growth stays game-side.
    Stars(u16),
    /// `call word near [slot]` — an indirect `cs:0x12xx` template
    /// dispatch the port doesn't specialise (arg in `di`/`dx`).
    Slot(u16, u16),
    /// `call word near [0x125A]` — the mode's draw-image-by-index
    /// routine, `dx` = image index (file `0x33A96`/`0x2A159`).
    Image(u16),
    /// `call 0x8577` — the sound-command wrapper: `ax` = command word,
    /// `cl` + 1 = driver function via `call far [cs:0x127C]` (file
    /// `0x38577`; gated on `[0x91D3] != 0xFF` inside the asm — the port
    /// applies the gate before emitting).
    Sfx(u16, u8),
    /// `int 0x33` — mouse driver call (`ax`/`cx`/`dx`; `ax = 8` sets
    /// the vertical range, `ax = 4` sets the cursor position).
    Mouse(u16, u16, u16),
    /// `call 0x37C0` — channel-A key pop: pulls a scancode/mouse-synth
    /// byte off the int-9 stack (`cs:0x8B72`), `0` when empty. The
    /// `0xE66E` line editor and `0xA1CA` menu path poll it.
    KeyPop,
}

/// Host callback — one method keeps call sites uniform; the return word
/// feeds the few services that answer in `ax`/`bl` (defaults `0`).
pub trait VmHost {
    /// Perform one host-side effect; return the `ax`/`bl` answer word.
    fn svc(&mut self, call: Call) -> u16;
    /// `svc` for calls whose asm body reads `ds:` cells the [`Call`]
    /// doesn't carry — the palette slots' `ds:[0x9CA1]` shade byte
    /// (`[0x125E]` present, `[0x128E]` fade). Forwards to `svc` — hosts
    /// with no palette model needn't see the state.
    fn svc_st(&mut self, call: Call, _vm: &Vm, _st: &State) -> u16 {
        self.svc(call)
    }
    /// Fetch a word from the code segment — the `cs:[si]` reads the
    /// draw sequencer at `cs:0x5C1E` makes. The shell maps `cs:` near
    /// offsets to file offsets (`+0x292B0`, the real `CS = 0x286B`
    /// base — not the `file − 0x30000` label space).
    fn cs_word(&mut self, _ofs: u16) -> u16 {
        0
    }
    /// Re-mirror the host's input state into the `ds:` latch cells —
    /// the port's stand-in for the int-9/int-33 handlers that feed
    /// `[0x9CCB]`/`[0x9CDA]` asynchronously. Runs at every `0x2CA9`
    /// frame boundary so blocking modal loops still see input.
    fn pump_input(&mut self, _vm: &mut Vm, _st: &mut State) {}
    /// Reset the channel-A int-9 stack — the `cs:[0x8B7C] = -1` the
    /// `0xE66E` editor entry writes (drops queued codes untouched).
    fn key_flush(&mut self) {}
}

/// A sink that drops every call — used when no frontend is attached.
pub struct NullHost;

impl VmHost for NullHost {
    fn svc(&mut self, _call: Call) -> u16 {
        0
    }
}
