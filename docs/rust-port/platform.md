# `platform` — DOS services layer

## What this module does

`platform` models the hardware/OS services the game logic calls: file
I/O (the `int 21h` wrappers), the system timer (the `int 8` PIT hook),
the keyboard (the `int 9` scancode ISR), and the mouse (`int 33h` plus
the `K`-flag keyboard emulation mode). Everything is expressed as plain
Rust calls — where the asm triggers an interrupt, the port invokes a
method; where the asm polls a device, the port reads a trait.

These four services are exactly the "host boundary" of 1991 DOS code:
the game logic in [game-state.md](game-state.md)–[game-dialogs.md](game-dialogs.md)
never touches hardware itself — it reads cells these services maintain
(`[0x9CCB]`/`[0x9CCC]` key latch, `[0x9CD6]`/`[0x9CD8]` cursor, the
LIFO stack) and asks them for work. This module is the bottom half of
the trait pattern described in [concepts.md](concepts.md) §9.

Ported from the surface documented in `decompiled/FUNCTIONS.md`.

## `DosFiles` (`files.rs`) — the `int 21h` wrappers

The only file I/O the game performs; each method maps one-to-one to a
DOS function (see [concepts.md](concepts.md) §3 for how `AH` selects the
service and the carry flag reports errors):

| method | int 21h | meaning |
|---|---|---|
| `find_first` | `AH=4E` | existence test (wildcards unused) |
| `open` | `AH=3D AL=0` | open read-only → handle |
| `read` | `AH=3F` | read → byte count (`0` = EOF) |
| `create` | `AH=3C CX=0` | create/truncate → handle |
| `open_write` | `AH=3D AL=1` | open write-only → handle |
| `write` | `AH=40` | write → byte count |
| `close` | `AH=3E` | close handle |

A quick primer on what an `int 21h` call looks like in the asm so the
table reads sensibly: `mov ah,0x3D; mov al,0; int 0x21` is "open
read-only" — `AH` picks the function, `AL` a sub-mode, `DS:DX` the
ASCIIZ name, and `AX` carries the result. The carry flag distinguishes
success/failure; the port's `Option`/`bool` is the same bit.

Handles start at 5 (0–4 are the standard devices stdin/stdout/stderr/
aux/prn). `None`/`false` mirrors the carry-flag error return the asm
tests.

Two implementors cover every use:

- `HostFs` (`hostfs.rs`) — `DosFiles` over `std::fs` rooted at a
  directory (the DOS cwd = the `GAME/` dir). Filenames resolve
  FAT-style: literal first, then uppercased, so case-sensitive hosts
  find the shipped `MCG.BIN`-style names. On a case-sensitive Linux/fs
  a name the game wrote lowercase still opens — DOS would have
  uppercased it at the FAT layer.
- `MemFs` (`memfs.rs`) — in-memory name→bytes map with fake handles for
  tests/headless runs; a `fail` flag forces the carry-flag error path on
  every op. `write` auto-extends the file at the handle's position, like
  a real DOS write. Because save/load goes through `DosFiles`, a `MemFs`
  test exercises the *entire* savegame path without touching disk —
  see [game-state.md](game-state.md) § `save.rs`/`load.rs`.

```rust
use supremacy::platform::DosFiles;   // trait must be in scope

let mut fs = supremacy::platform::HostFs::new("GAME".into());
assert!(fs.find_first("MCG.GPH"));
if let Some(h) = fs.open("MCG.GPH") {
    let mut buf = [0u8; 12];
    let n = fs.read(h, &mut buf);          // 12 on success, 0 at EOF
    fs.close(h);
}
```

## `Keyboard` (`keyboard.rs`) — the int-9 ISR

Model of the scancode ISR at file `0x31D74` (`286B:8AC4`, the first far
pointer in every mode template). On a real PC the keyboard fires int 9
per keypress and the ISR reads port `0x60`, ACKs via the `0x61` bit-7
toggle, EOIs the PIC, then:

- tracks Ctrl/Alt/Del (`0x1D`/`0x38`/`0x53` + breaks `0x9D`/`0xB8`/`0xD3`)
  in `[cs:0x8B6F..0x8B71]` — all three held does `jmp 0xFFFF:0`
  (`modifiers` sets `reboot`; `kbd_push.rs`) — the 1991
  Ctrl+Alt+Del reboot, a real BIOS warm-boot target;
- latches the raw code into `last` (`[0x9CCC]`) and sets `pending`
  (`[0x9CCB]` — what the K-mode mouse path polls);
- pushes the code onto a **10-entry LIFO stack** (`cs:0x8B72`, count at
  `cs:0x8B7C`, `-1` empty, push skipped at 9 — BIOS-style drop-on-full).

Two things worth noting for a reader new to this era. First, the game
receives **scancodes**, not characters — hardware key positions (set 1);
`0x1C` is Enter whatever its label says, and a *release* is the same
code `| 0x80` (a "break"). Second, the stack is a **LIFO**, which is
unusual — the BIOS's own keyboard buffer is a FIFO. The game *wants*
newest-first: a fast double-tap reads back in reverse order, and
`line_input`'s debounce relies on it. The port keeps the LIFO because
consumers' behaviour depends on it.

`pop()` (`kbd_pop.rs`, file `0x31E2E`) returns the newest code — LIFO,
not the BIOS FIFO, which is why a fast double-tap reads back newest
first. `take_pending()` consumes the `[0x9CCB]`/`[0x9CCC]` latch the
menu wait loops poll — the latch is the "one most recent keypress"
channel, the stack the "everything since you last drained" channel.

`nav_dir(scan, tga)` (`kbd_nav.rs`) — arrows `0x48`/`0x50`/`0x4B`/`0x4D`
→ up/down/left/right (`+0x10..+0x13` hotspot fields); Tandy mode
(`[0x9CC6] == 1`) additionally accepts `0x29`/`0x4A`/`0x2B`/`0x4E` —
the Tandy 1000's extra cursor keys, which emit different codes on that
hardware.

## `Mouse` (`mouse.rs`) — int-33h state

`x`/`y`/`buttons` mirror `ds:[0x9CD6]`/`[0x9CD8]`/`[0x9CDA]`;
`keyboard_emulated` is the `K`-arg flag (`[0x9CCD]` — see
[args.md](args.md)). Under the hood the real DOS mouse driver (usually
`MOUSE.COM`, loaded before the game) receives mouse hardware IRQs and
calls a *user callback* the game registered via `int 33/ax=0x14` —
that callback is what the `0x7E`/`0x7D`/`0xFE`/`0xFD` synthetic codes
below simulate. `K` mode swaps that whole path for keyboard-driven
emulation (arrows glide the cursor, Enter/Esc act as the buttons).

`feed_scan` (`mouse_feed.rs`) — the button-event synth from files
`0x2A18E` (real driver) and `0x2A1CD` (`K` mode): the scancode a real
mouse driver or the keyboard-emulation path would turn into a button
change.

| code | K mode | real driver |
|---|---|---|
| left down | `0x1C` (Enter) | `0x7E` |
| right down | `0x01` (Esc) | `0x7D` |
| release | `0x9C`/`0x81` breaks | `0xFE`/`0xFD` |

Produces `MouseEvent::{LeftDown,RightDown,Release}` and updates
`buttons` to `1`/`2`/`0`. `set()` (`mouse_move.rs`) writes the full
int33-style state at once — the port's equivalent of an `int 33/ax=4`
position set.

## `Timer` (`timer.rs`) — the int-8 PIT hook

Ports file `0x353D`–`0x35BF`: PIT channel 0 reprogrammed to mode 3
(`out 0x43=0x36`, divisor to `0x40`); the ISR EOIs the PIC every pulse
and chains to the saved BIOS handler every 3rd tick — the game runs its
frame pacing at the reprogrammed rate while DOS keeps its standard
18.2 Hz clock (see [concepts.md](concepts.md) §3 for how a PIT divisor
maps to a frequency).

How the pacing works: the PIT counts down from `divisor` at a fixed
1.193182 MHz input clock, so `tick rate = 1193182 / divisor` Hz. A
divisor of `0x5CEC` (= 23788) gives ≈ 50.17 Hz — the game ticks its
sound drivers ~50 times a second instead of DOS's default 18.2. But DOS
itself depends on int 8 firing at 18.2 Hz for its clock — so the ISR
swallows two pulses out of three (`phase`) and only chains to the saved
BIOS handler on the third: the game sees 50 Hz, DOS sees 18.2 Hz,
neither knows the difference.

- `program(divisor)` (`timer_program.rs`) — `divisor` written to
  port `0x40`; `0` = 65536 ≈ 18.2 Hz (also the uninstall value —
  `timer_uninstall` passes 0 to restore the BIOS rate).
- `tick()` (`timer_tick.rs`) — one PIT pulse: decrements the
  `[CS:0x1BFE]` down-counter (`phase`); every 3rd pulse it reloads to 3
  and returns `true` = chain to the BIOS int-8 via `pushf`/`call far`.
- `freq_mhz()` (`pit_freq.rs`) — tick rate in millihertz:
  `1_193_182_000 / divisor` (PIT input 1.193182 MHz; `0` → 65536).
  e.g. `freq_mhz()` on `0x5CEC` ≈ 50170 mHz — the ~50 Hz the drivers
  pace at.

## Pitfalls

- **`DosFiles` errors are `Option`/`bool`, not `Result`.** `None` and
  `false` are the carry flag; there is no richer error to preserve —
  match the asm by treating them as failed, not exceptional.
- **The keyboard LIFO inverts intuition.** `pop` returns the *newest*
  code. Tests that push `A` then `B` get `B` back first — that is the
  original's behaviour, kept deliberately.
- **`K` mode is a mode, not an input.** `[0x9CCD]` selects which
  *path* synthesises button events; the menu service's `nav`/`snap`
  cursor glide only exists on the `K` path.
- **`Timer::tick` returning `true` ≠ "tick happened".** `true` marks
  the every-third-pulse BIOS chain; the game's own work ran on *every*
  pulse regardless.

## File map

| file | item |
|---|---|
| `files.rs` | `DosFiles` trait (the `int 21h` surface) |
| `hostfs.rs` | `HostFs` — real filesystem, FAT-style name resolution |
| `memfs.rs` | `MemFs` — in-memory filesystem for tests |
| `keyboard.rs` | `Keyboard` — the int-9 ISR state (`buf`, `count`, `mods`, `last`, `pending`, `reboot`) |
| `kbd_push.rs` | `irq`/`push`/`modifiers` — the ISR body |
| `kbd_pop.rs` | `pop`/`take_pending` — LIFO drain, latch consume |
| `kbd_nav.rs` | `nav_dir` — scancode → `NavDir` (+ Tandy extras) |
| `mouse.rs` | `Mouse` state + `MouseEvent` |
| `mouse_feed.rs` | `feed_scan` — scancode → button event |
| `mouse_move.rs` | `set` — int-33 position/buttons write |
| `timer.rs` | `Timer` — `divisor`, `phase` |
| `timer_new.rs` | `Timer::new` + `CHAIN_EVERY = 3` |
| `timer_program.rs` | `program` — `out 0x43=0x36`, lo/hi `0x40` |
| `timer_tick.rs` | `tick` — one PIT pulse, the 3-phase chain |
| `pit_freq.rs` | `freq_mhz` + `PIT_HZ` |

## Worked example — a keypress's journey

User presses Enter: `Keyboard::irq(0x1C)` → `modifiers` (not a
modifier, no-op) → `last = 0x1C`, `pending = true` → `push`: `count`
goes −1 → 0, `buf[0] = 0x1C`. The menu loop's `take_pending()` reads
`0x1C`, clears `pending`; `pop()` returns `buf[0]` and `count` → −1
(empty). A release produces `irq(0x9C)` — same path, the `0x80` bit
marks it a break, which `feed_scan`'s K-mode table maps to a button
release.
