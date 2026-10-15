# `game` (5/6) — display services: text, numbers, blink, hover

Page 5 of the `game` module docs — the small display routines the
frame/shell pages ([game-ui.md](game-ui.md)) call every frame: the
print interpreter, the decimal printers, the blink phase and the oval
hover test. See [concepts.md](concepts.md) for `ds:`/`cs:` notation.

These are the game's *primitive* text/graphics verbs — the routines
every other page's routines call when they want output. A `Call::Glyph`
from here is what eventually lands as pixels through
[front.md](front.md)'s font renderer.

## `text.rs` — the `cs:0x6709` print interpreter (`0x36709`–`0x36791`)

`print_str(si, bx, dx)`: walks a string at `ds:si` drawing glyphs at
character-cell coordinates — `(bx·4px, dx)` because the font is 4 px
wide and coordinates are in 4-px columns (`bx += 1` per char). The
string isn't ASCII-with-terminator — it's a tiny bytecode with inline
move/fill escapes:

| byte | meaning |
|---|---|
| `0xFF` | end of string |
| `≥ 0x20` | draw glyph `b` via `Call::Glyph`, `bx += 1` |
| `0x01` / `0x02` | sign-extend next byte into `dx` / `bx` — inline "move pen" |
| `0x03` / `0x04` | read `{count, char}` — repeat `count` times across (`bx`) / down (`dx += 6`) — run-length fill for rules and boxes |
| other | terminate |

So `ds:… 48 45 4C 4C 4F FF` prints "HELLO" and `… 03 0A 2D …` draws a
run of 10 `0x2D` (`-`) glyphs — which is how the panel borders and
rules get drawn without a draw-rect call. The `0x01`/`0x02` escapes
are how a single string can address a second line without splitting
into two strings.

`put` = the shared `0x68D6` glyph put. `pace` — while
`[0x91EA] != 0` every char also runs the typewriter block (`seq_step`,
`tick_step`, `vsync`, `PANEL_ON && uimode==5` overlay) —
`0x3672A`–`0x36744`: text prints one character per frame tick, which is
the typewriter effect on story screens. The important consequence:
**with `OVERLAY_F` set, `print_str` is not synchronous** — it interleaves
frame work between glyphs, so the string "types" over many frames.

`enqueue(si)` (`cs:0x6B06`, `0x36B06`–`0x36B49`) — ticker message
enqueue: sound select (`Chan(0x8A5B,0)`/`Chan(0x8A3B,0x1F)`/
`Sound(0xC)`, muted when `[0x91D3]==0xFF`), `0x91E8 = 0xFF`, append
`{si, 0}` at `[0x9160]` (`0xFE` wrap to `0x928D`), `[0x91C2] += 1`
(skipped at `0x80`) — the queue the status ticker drains (see
[game-tick.md](game-tick.md) § `ticker.rs`). The queue is *addresses*,
not bytes: a ticker entry is "the string at `ds:si`", and the ticker
walks it one byte per step — so the string must stay live for as long
as the message is queued.

## `num.rs` — decimal printers (`0x3686C`–`0x36A7A`)

`num_pad` (`cs:0x686C`, u16 padded to 5 cols), `num` (`cs:0x698F`,
unpadded — `0` still prints `'0'`), `num32` (`cs:0x68E0`, `bp:ax` u32
padded to 8). `field` walks the decade tables (`DEC16`/`DEC32` —
`10000, 1000, 100, 10, 1`): leading decades emit `0x20` (padding
variants) while `v < d`; once a digit lands every later position
prints — the standard "suppress leading zeros by spaces" routine. All
emit `0x68D6` char puts at `(bx·4px, dx)`, `bx += 1` per char; returns
`bx` advanced.

Worth noting the callers' convention: the printed column is `bx` (4-px
units), so `num_pad` at `bx = 0x45` prints at pixel `0x114`. The
"suppress leading zeros" behaviour is why the score readouts look
right-aligned — the spaces blank whatever was under the empty columns.

## `blink.rs` — `blink_step` (`cs:0x6B4A`, `0x36B4A`–`0x36B8E`)

Per shell frame — the flashing "press a key" prompt. `BLINK_T==0` →
tail-fall into `ticker_step` (the blink routine *is* the ticker's caller
in the shell loop — the shell calls `blink_step`, and the ticker rides
its tail when idle). While armed: `BLINK_HOLD` suppresses the phase
machinery (just `dec` + `Slot(0x1270)`); otherwise `BLINK_PH` counts up
to `0x11`, at which point `BLINK_T` stops decrementing and the routine
alternates the `0x77AA`/`0x77E3` strings at `(0x4D,0x67)` on `FRAME&7`
phases 0/5 — draw one string, hold five frames, draw the other.

The string pair is what makes the prompt *flash*: `0x77AA` and
`0x77E3` are the same message in different colours (or a message and
its blank), alternated every few frames. `BLINK_HOLD` exists so the
dialogs can run the ticker without the blink machinery fighting over
the same lines — `dialog::pump` sets it while it drains the queue.

## `hover.rs` — `hover_check` (`cs:0x8DC6`, `0x38DC6`–`0x38E0F`)

Oval region centred `x=0x50`, `y∈[0x40,0x9B)` — the planet oval on the
galaxy map. The table index is `bx + 0xA8` into the runtime-loaded
half-width table `ds:0xA8` (the shipped image has zeros — the table is
*populated at runtime*, another case of the port trusting the image to
be live), where `bx` is
`y` (under `0x40`), `0x3F−y` (`0x40..=0x50`) or `0x90−y` (`0x51..0x9B`)
— all 16-bit wrapping, so the same table indexes the oval from either
edge. `x` is tested against `0x50 ∓ w` (8-bit wrap on both bounds,
`x+8 ≥ lo`). Inside → the frame's generic handler is skipped; outside
→ `Call::Slot(0x1278, 0)` — the map redraw slot repaints the star map
when the cursor leaves the oval.

Read the table trick as: the oval is symmetric about `y = 0x50`, so
`bx` collapses each half to the same `0..=0x11` index — the half-width
for `y` is the half-width for `0xA0 − y`. The 8-bit wrap on the bounds
isn't a bug to fix — it's how the asm handles `x` near the screen edge
without a wider compare, and the port reproduces it.

## Pitfalls

- **String coordinates are in 4-px columns**, not pixels — `bx` counts
  glyphs, so `bx = 0x45` is pixel `0x114`.
- **`print_str` can take many frames** under `OVERLAY_F`; tests that
  call it once see only the first character's work.
- **Ticker entries are pointers.** A message must stay at `ds:si`
  until dequeued — rotating a string's storage mid-queue corrupts the
  walk.
- **`hover` reads a runtime-built table.** `ds:0xA8` is zeros in the
  shipped image; the test must load it (or the ports' `Host` does) for
  the oval test to mean anything.

## File map

| file | item |
|---|---|
| `text.rs` | `print_str`, `put`, `pace`, `enqueue` — the `0x6709` interpreter + ticker enqueue |
| `num.rs` | `num_pad`, `num`, `num32`, `field` — the decimal printers |
| `blink.rs` | `blink_step` — the flashing prompt phase |
| `hover.rs` | `hover_check` — the planet-oval hit test |

## Worked example — a `print_str` string

The bytes `48 49 01 F0 03 05 2D FF` print:

- `48 49` → glyphs 'H','I' at `bx`, `bx+1` (columns, ×4 px).
- `01 F0` → `dx += −16` (sign-extended): move pen up two rows.
- `03 05 2D` → repeat `0x2D` ('-') 5 times across — a rule.
- `FF` → end.

One call draws "HI", a line break, and a 5-char rule — the format's
compression for borders.
