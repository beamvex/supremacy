# `game` (5/6) — display services: text, numbers, blink, hover

## `text.rs` — the `cs:0x6709` print interpreter (`0x36709`–`0x36791`)

`print_str(si, bx, dx)`: `0xFF` ends; `≥0x20` draws a glyph via
`Call::Glyph` at `(bx·4px, dx)`, `bx += 1`; `0x01`/`0x02` sign-extend
the next byte into `dx`/`bx`; `0x03`/`0x04` read a `{count, char}` word
and repeat (`bx += 1` across / `dx += 6` down); other control bytes
terminate. `put` = the shared `0x68D6` glyph put. `pace` — while
`[0x91EA] != 0` every char also runs the typewriter block (`seq_step`,
`tick_step`, `vsync`, `PANEL_ON && uimode==5` overlay) —
`0x3672A`–`0x36744`.

`enqueue(si)` (`cs:0x6B06`, `0x36B06`–`0x36B49`) — ticker message
enqueue: sound select (`Chan(0x8A5B,0)`/`Chan(0x8A3B,0x1F)`/
`Sound(0xC)`, muted when `[0x91D3]==0xFF`), `0x91E8 = 0xFF`, append
`{si, 0}` at `[0x9160]` (`0xFE` wrap to `0x928D`), `[0x91C2] += 1`
(skipped at `0x80`).

## `num.rs` — decimal printers (`0x3686C`–`0x36A7A`)

`num_pad` (`cs:0x686C`, u16 padded to 5 cols), `num` (`cs:0x698F`,
unpadded — `0` still prints `'0'`), `num32` (`cs:0x68E0`, `bp:ax` u32
padded to 8). `field` walks the decade tables (`DEC16`/`DEC32`): leading
decades emit `0x20` (padding variants) while `v < d`; once a digit lands
every later position prints. All emit `0x68D6` char puts at `(bx·4px,
dx)`, `bx += 1` per char; returns `bx` advanced.

## `blink.rs` — `blink_step` (`cs:0x6B4A`, `0x36B4A`–`0x36B8E`)

Per shell frame. `BLINK_T==0` → tail-fall into `ticker_step`. While
armed: `BLINK_HOLD` suppresses the phase machinery (just `dec` +
`Slot(0x1270)`); otherwise `BLINK_PH` counts up to `0x11`, at which
point `BLINK_T` stops decrementing and the routine alternates the
`0x77AA`/`0x77E3` strings at `(0x4D,0x67)` on `FRAME&7` phases 0/5 —
the flashing "press" prompt.

## `hover.rs` — `hover_check` (`cs:0x8DC6`, `0x38DC6`–`0x38E0F`)

Oval region centred `x=0x50`, `y∈[0x40,0x9B)`: the table index is
`bx + 0xA8` into the runtime-loaded half-width table `ds:0xA8` (the
shipped image has zeros), where `bx` is `y` (under `0x40`),
`0x3F−y` (`0x40..=0x50`) or `0x90−y` (`0x51..0x9B`) — all 16-bit
wrapping. `x` is tested against `0x50 ∓ w` (8-bit wrap on both bounds,
`x+8 ≥ lo`). Inside → the frame's generic handler is skipped; outside
→ `Call::Slot(0x1278, 0)`.
