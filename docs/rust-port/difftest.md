# `difftest` — DOSBox-X differential testing (GAM-25)

## What this is for

The strongest verification tool in the repo: it runs the **original
`GAME.EXE` under DOSBox-X** and the Rust port on the *same* scripted
input, dumps guest memory/palette/state from both, and diffs the bytes —
GAM-9's "run original vs port side-by-side on scripted input and compare
state" stretch. When the port disagrees with the original, the diff
names the exact byte that diverged.

Think of it as a two-player replay: a TSR probe inside the emulator and
the port-side driver both consume the same op schedule (keys, mouse
moves, pokes, waits, dumps), then `report.txt` compares the artefacts
byte-for-byte.

Why this and not just unit tests? Unit tests prove the port does what
we *think* the asm does; difftest proves it does what the asm *actually
does*, including the cases nobody thought to test — the only honest
referee is the original binary.

## Pieces

- `difftest/probe.asm` → `difftest/probe.com` (`nasm -f bin`). A TSR
  loaded before the game: hooks int 21h (segment discovery), int 33h
  (mouse-handler capture) and the game's int 8h timer chain (tick +
  scheduler). One tick = one int-8 chain call ≈ 55–60 ms — the probe's
  notion of a frame.

  A TSR (terminate-and-stay-resident) is a 1991-era resident program:
  it installs hooks into the interrupt vectors, then exits while keeping
  its memory — so it keeps running inside the game process, unseen.
  The probe needs to be one because it has to live *underneath* the
  game, observing, without the game knowing. When the script says `key`
  the probe injects the scancode; when it says `dump` the probe copies
  guest memory to a file — from inside DOS, with the game's own `ds`
  in scope, which is the only way to see `ds:0x9CCB`-style cells.

- `rust/src/difftest/` — `op`/`script` (record codec + text parser),
  `seg` (`ds`/`drv`/absolute selectors), `dump` (artefact names +
  24-byte debug block), `snap` (port-side byte images), `input` (the
  key/mouse/poke/wait op application, shared with `smoke`), `drive`
  (the replay loop), `boot` (start the port from a DOS `ds` dump), `conf`
  (the generated `dosbox-x.conf`), `spawn` (headless DOSBox-X),
  `report` (the diff).
- `rust/src/bin/difftest.rs` — the driver binary.
- `rust/tests/difftest.rs` — codec/replay/boot tests plus a gated E2E.

## Script

Text, one op per line, hex fields (see `script.rs`'s doc header). The
CLI writes it as `difftest/SCRIPT.BIN` — fixed 12-byte records
`[u16 tick][u8 op][u8 a][u16 b][u16 c][u16 d][u16 pad]`:

| op | fields |
|---|---|
| `key` | `a` = scancode (bit 7 = break) — mirrors `Pump::key`: `[0x9CCB]`/`[0x9CCC]` latch + `drv:0x8B72` LIFO push |
| `mouse` | `a` mask, `b` x, `c` y, `d` buttons — far-calls the game's int-33h event handler |
| `dump` | `a` file id, `b` ofs, `c` len, `d` seg — writes `OUT\Dxx.BIN` |
| `pokew` | `b` ofs, `c` word, `d` seg |
| `pal` | `a` file id — 768-byte VGA DAC to `Pxx.BIN` |
| `stat` | `a` file id — 24-byte probe debug block to `Sxx.BIN` |
| `waitcell` | `b` ds ofs, `c` byte — blocks until `ds:[b] == c` |
| `done` | writes `DONE.BIN`, the host's end marker |

`seg`: `ds`/`0xFFFF` = game dgroup, `drv`/`0xFFFE` = driver segment
(LIFO + `0x55A2` LCG state), otherwise absolute (`A000` = VRAM). The
selector indirection exists because **segment values aren't stable
across DOS loads** — the game might land at `ds = 0x1A23` in one boot
and a different value in another, so a script that said
`dump A000 0 FA00 ds` would mean different physical memory on different
runs. `ds:`/`drv:` selectors are resolved by the probe to whatever the
running game actually has.

A minimal script that boots, waits for the shell (`uimode` = 0), snaps
the state block, and exits:

```
waitcell 91CD 00          # hold until uimode == 0 (shell)
dump 01 7D39 1F8B ds      # D01.BIN = the whole saveable block
pal  02                   # P02.BIN = the live DAC
stat 03                   # S03.BIN = the probe's debug block
done
```

Segments are discovered at runtime — the `0xFAA` in the int-9 ISR is a
relocation site (observed `0x1A23`), so nothing is hardcoded.

## Running

```bash
nasm -f bin -o difftest/probe.com difftest/probe.asm
cargo run --bin difftest -- scenario.scr --root .. --timeout 240
```

DOSBox-X is located via `DOSBOX_X`, `PATH`, or usual install spots;
`--port-only` skips the DOS pass (port dumps + report vs whatever is in
`out/`). SDL's dummy video/audio drivers keep it headless for CI. The
run writes `difftest/out/Dxx.BIN`/`Pxx.BIN`/`Sxx.BIN`/`ARMED.BIN`/
`DONE.BIN` on the DOS side and mirrors under `difftest/out/port/`;
`report.txt` diffs them byte-for-byte and lists the first eight
mismatches per artefact.

Reading a report: each artefact gets a line (`D01.BIN: 128/8067 bytes
differ, first @ 0x024`), then the first few mismatching offsets with
both sides' values — the offset is inside the artefact, so
`D01 + 0x024` = `ds:0x7D39 + 0x24` = `ds:0x7D5D` — which you then look
up in [game-state.md](game-state.md)'s cell map.

## Checkpoints and timing skew

The game's `ds:0x91D4` frame cell doesn't free-run in the shell, so the
probe schedules on its own tick; `waitcell` gives exact sync on a game
byte (e.g. `waitcell 91CD 5` = hold until `uimode` = galaxy). The port
maps one tick to one `Game::step`, and skew is absorbed by comparing at
quiescent checkpoints — dumps scheduled after the machines on both
sides have settled, not mid-animation.

This is the practical rule: **only diff quiescent state**. Two machines
running the same algorithm at different rates will legitimately be
mid-way through different work at any instant; `waitcell` + a settled
frame count is how you pick instants where they must agree.

## Booting the port from a DOS dump (the RNG problem)

For runs beyond boot the RNG matters — `new_game` consumes the LCG and
the original reseeds it from the timer, so the two sides' seeds can
never coincide. A script therefore typically dumps `ds`
(`0..0x8000`, `0x8000..0x10000` = `D01`/`D02`) and the `drv:0x55A2`
dword (`D03`) early; `boot` then restores the port from those dumps
(`Rng::set_state`) and replays only ops scheduled after `boot_tick`,
making later dumps comparable without needing the seeds to coincide.
The port doesn't need to *reproduce* the original's random boot — it
needs to *continue identically from an identical state*.

Concretely: run the DOS side past boot, snapshot its `ds` + RNG state,
boot the port into that exact image, then replay the rest of the script
on both. From the snapshot tick onward, both machines see the same
inputs and (if the port is faithful) reach the same dumps.

The E2E test is opt-in — `DIFFTEST_E2E=1` plus a `dosbox-x` binary and
`difftest/probe.com`; it writes into `difftest/out/`.

## Pitfalls

- **Dumps mid-animation lie.** Always schedule artefacts after a
  `waitcell`/`settle` window; a `D01` taken while a machine is still
  ticking will differ on both sides for boring reasons.
- **`ds:`/`drv:` are selectors, not segments.** Writing `d=0x1A23`
  bakes in today's DOS load address; `d=ds` asks the probe.
- **Bootstrapping requires the `D03` RNG word.** Without it, the port
  boots with a different seed and the *lawful* divergence is
  indistinguishable from a bug.
- **`--port-only` compares stale artefacts.** It replays the port side
  against whatever DOS-side files are already in `out/` — regenerate
  them, don't trust last week's.

## File map

| file | item |
|---|---|
| `op.rs` | the `Op` record codec (12-byte wire format) |
| `script.rs` | text `.scr` parser → `Op` list |
| `seg.rs` | `ds`/`drv`/absolute segment selectors |
| `dump.rs` | artefact names + the 24-byte debug block |
| `snap.rs` | port-side byte-image snapshots |
| `input.rs` | `key`/`mouse`/`pokew`/`waitcell` application (shared with smoke) |
| `drive.rs` | the replay loop over a script |
| `boot.rs` | port boot from DOS `ds` dumps (`Rng::set_state`) |
| `conf.rs` | generated `dosbox-x.conf` |
| `spawn.rs` | headless DOSBox-X subprocess |
| `report.rs` | byte-for-byte artefact diff |

## Writing a scenario, step by step

A scenario that opens the shell, clicks hotspot 0, and diffs state:

```
waitcell 91CD 00           # uimode == 0 — shell is live
mouse 00 0064 0030 01      # left down at (0x64,0x30)
key   9C                   # … (example: a release scancode if needed)
waitcell 91CA 00           # ARMED cleared — click consumed
dump  04 7D39 1F8B ds      # state block, post-click
pal   05                   # DAC after whatever the click repainted
done
```

The discipline: every `dump`/`pal` is preceded by a `waitcell` on a
cell that proves the interesting work settled — click → `ARMED`,
dialog → `DLG_FLAG`, screen change → `UIMODE`. Without those gates both
sides legitimately differ mid-frame.

## What `report.txt` can't tell you

A byte-level diff says *where* the sides diverged, not *why* — two
cells may differ because a third one fed them. The workflow when a
diff surprises you: take the first differing offset, map it to the cell
(`D01` + ofs → `ds:` offset → [game-state.md](game-state.md)'s map),
then bisect the script — dump the same artefact one op earlier until
the divergence appears, and the op that introduces it is your bug.
