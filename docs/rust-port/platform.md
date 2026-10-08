# `platform` — DOS services layer

The hardware/OS services the game logic calls, ported from the surface
documented in `decompiled/FUNCTIONS.md`.

## `DosFiles` (`files.rs`) — the `int 21h` wrappers

The only file I/O the game performs:

| method | int 21h | meaning |
|---|---|---|
| `find_first` | `AH=4E` | existence test (wildcards unused) |
| `open` | `AH=3D AL=0` | open read-only → handle |
| `read` | `AH=3F` | read → byte count (`0` = EOF) |
| `create` | `AH=3C CX=0` | create/truncate → handle |
| `open_write` | `AH=3D AL=1` | open write-only → handle |
| `write` | `AH=40` | write → byte count |
| `close` | `AH=3E` | close handle |

Handles start at 5 (0–4 are standard devices). `None`/`false` mirrors
the carry-flag error return.

- `HostFs` (`hostfs.rs`) — `DosFiles` over `std::fs` rooted at a
  directory (the DOS cwd = the `GAME/` dir). Filenames resolve FAT-style:
  literal first, then uppercased, so case-sensitive hosts find the
  shipped `MCG.BIN`-style names.
- `MemFs` (`memfs.rs`) — in-memory name→bytes map with fake handles for
  tests/headless runs; a `fail` flag forces the carry-flag error path on
  every op. `write` auto-extends the file at the handle's position.

## `Keyboard` (`keyboard.rs`) — the int-9 ISR

Model of the scancode ISR at file `0x31D74` (`286B:8AC4`, the first far
pointer in every mode template): reads port `0x60`, ACKs via the `0x61`
bit-7 toggle, EOIs the PIC, then:

- tracks Ctrl/Alt/Del (`0x1D`/`0x38`/`0x53` + breaks) in
  `[cs:0x8B6F..0x8B71]` — all three held does `jmp 0xFFFF:0`
  (`modifiers` sets `reboot`; `kbd_push.rs`);
- latches the raw code into `last` (`[0x9CCC]`) and sets `pending`
  (`[0x9CCB]` — what the K-mode mouse path polls);
- pushes the code onto a **10-entry LIFO stack** (`cs:0x8B72`, count at
  `cs:0x8B7C`, `-1` empty, push skipped at 9 — BIOS-style drop-on-full).

`pop()` (`kbd_pop.rs`, file `0x31E2E`) returns the newest code (LIFO,
not the BIOS FIFO); `take_pending()` consumes the `[0x9CCB]`/`[0x9CCC]`
latch the menu wait loops poll.

`nav_dir(scan, tga)` (`kbd_nav.rs`) — arrows `0x48`/`0x50`/`0x4B`/`0x4D`
→ up/down/left/right (`+0x10..+0x13` hotspot fields); Tandy mode
(`[0x9CC6] == 1`) additionally accepts `0x29`/`0x4A`/`0x2B`/`0x4E`.

## `Mouse` (`mouse.rs`) — int-33h state

`x`/`y`/`buttons` mirror `ds:[0x9CD6]`/`[0x9CD8]`/`[0x9CDA]`;
`keyboard_emulated` is the `K`-arg flag (`[0x9CCD]`).

`feed_scan` (`mouse_feed.rs`) — the button-event synth from files
`0x2A18E` (real driver) and `0x2A1CD` (`K` mode):

| code | K mode | real driver |
|---|---|---|
| left down | `0x1C` (Enter) | `0x7E` |
| right down | `0x01` (Esc) | `0x7D` |
| release | `0x9C`/`0x81` breaks | `0xFE`/`0xFD` |

Produces `MouseEvent::{LeftDown,RightDown,Release}` and updates
`buttons` to `1`/`2`/`0`. `set()` (`mouse_move.rs`) writes the full
int33-style state at once.

## `Timer` (`timer.rs`) — the int-8 PIT hook

Ports file `0x353D`–`0x35BF`: channel 0 reprogrammed to mode 3
(`out 0x43=0x36`, divisor to `0x40`); the ISR EOIs the PIC every pulse
and chains to the saved BIOS handler every 3rd tick.

- `program(divisor)` (`timer_program.rs`) — `divisor` written to
  port `0x40`; `0` = 65536 ≈ 18.2 Hz (also the uninstall value).
- `tick()` (`timer_tick.rs`) — one PIT pulse: decrements the
  `[CS:0x1BFE]` down-counter (`phase`); every 3rd pulse it reloads to 3
  and returns `true` = chain to the BIOS int-8 via `pushf`/`call far`.
  The game ticks at the reprogrammed rate while DOS keeps 18.2 Hz.
- `freq_mhz()` (`pit_freq.rs`) — tick rate in millihertz:
  `1_193_182_000 / divisor` (PIT input 1.193182 MHz; `0` → 65536).
