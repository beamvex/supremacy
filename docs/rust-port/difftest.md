# `difftest` — DOSBox-X differential testing (GAM-25)

Runs the original `GAME.EXE` under DOSBox-X and the port on the same
scripted input, then diffs guest state — GAM-9's "run original vs port
side-by-side on scripted input and compare state" stretch.

## Pieces

- `difftest/probe.asm` → `difftest/probe.com` (`nasm -f bin`). A TSR
  loaded before the game: hooks int 21h (segment discovery), int 33h
  (mouse-handler capture) and the game's int 8h timer chain (tick +
  scheduler). One tick = one int-8 chain call ≈ 55–60 ms.
- `rust/src/difftest/` — `op`/`script` (record codec + text parser),
  `seg` (`ds`/`drv`/absolute selectors), `dump` (artefact names +
  24-byte debug block), `snap` (port-side byte images), `drive` (the
  replay loop), `boot` (start the port from a DOS `ds` dump), `conf`
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
(LIFO + `0x55A2` LCG state), otherwise absolute (`A000` = VRAM).

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

## Checkpoints

The game's `ds:0x91D4` frame cell doesn't free-run in the shell, so the
probe schedules on its own tick; `waitcell` gives exact sync on a game
byte (e.g. `waitcell 91CD 5` = hold until `uimode` = galaxy). The port
maps one tick to one `Game::step`, and skew is absorbed by comparing at
quiescent checkpoints.

For runs beyond boot the RNG matters — `new_game` consumes the LCG and
the original reseeds it from the timer, so a script typically dumps
`ds` (`0..0x8000`, `0x8000..0x10000` = `D01`/`D02`) and the
`drv:0x55A2` dword (`D03`) early; `boot` then restores the port from
those dumps (`Rng::set_state`) and replays only ops scheduled after
`boot_tick`, making later dumps comparable without needing the seeds
to coincide.

The E2E test is opt-in — `DIFFTEST_E2E=1` plus a `dosbox-x` binary and
`difftest/probe.com`; it writes into `difftest/out/`.
