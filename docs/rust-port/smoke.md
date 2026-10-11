# `smoke` — headless CI mode (GAM-24)

GAM-9's "run N ticks deterministically and hash the framebuffer/audio
state so regressions are testable without a display". The
`supremacy-smoke` binary builds with default features — no `minifb`,
no `cpal` — so a CI runner with no windowing or audio device can compile
and run it.

## Running

```bash
cargo run --bin supremacy-smoke -- ../GAME --ticks 300
# smoke steps=300 video=D2E728DEC150233C pal=179DCB6EB754B64F
#       audio=F02A2B9E5641D61B state=F891E7DC4E7F4001 rng=00023B6B
```

Args: `<GAME-dir> [V] [K] [S]` positionally (the same PSP-tail letters
as `supremacy`/`supremacy-run`), plus `--set STEM`, `--exe PATH`,
`--preset N`, `--seed N`, `--galaxy`, `--ticks N`, `--script F.scr`,
`--expect "k=v,…"`, `--frame F.ppm`.

The boot is `run.rs`'s: unpacked exe (`--exe`, default
`<dir>/../decompiled/GAME.unpacked.exe`), one asset set, the mode
palette, `Game::new` over a `MemFs` (savegames never touch disk),
`set_assets` for `Call::Load`, `--galaxy` for `enter_galaxy`.

## The digest

`smoke::Digest` — five channels so a failure names its subsystem:

| field | content |
|---|---|
| `steps` | `Game::step`s run (wait-loop steps an op consumed count too) |
| `video` | FNV-1a of `Host::scr.buf` (mode-laid-out VRAM) |
| `pal` | FNV-1a of the 768 palette bytes |
| `audio` | FNV-1a of every rendered PCM `f32` — `SAMPLES_PER_STEP`
  (`BOOT_RATE`/30 = 1470) per step, the window loop's stride |
| `state` | FNV-1a of `difftest::ds_image` (the reassembled 64 KiB `ds`) |
| `rng` | the final `Rng` LCG state, raw `u32` |

Determinism is the contract — same exe image, assets, args, seed and
script must always produce the same line. Steps run inside an op's
wait (`waitcell`, a busy `[0x9CCB]` latch) count toward `steps` but
render no audio: same input, same render points, still deterministic.

## Scripted input

`--script` takes the `difftest` text format — ops apply on their tick
through the shared `difftest::input` (identical `Pump`/poke/wait
semantics as the DOSBox-X harness): `key`, `mouse`, `pokew`,
`waitcell`; `done` ends the run early. The artefact ops (`dump`,
`pal`, `stat`) are no-ops here — the hash channels are the snapshot.

## Gating

`--expect` takes `key=value` pairs separated by commas or whitespace
(`steps` decimal, the rest hex, `0x` tolerated) and exits 3 on any
mismatch, printing `expect: <field>: got X, want Y` per failure —
pin the full line or any subset. Exit 0 = match; 1 = asset/exe load
failure; 2 = usage.

```bash
supremacy-smoke GAME --ticks 300 \
    --expect "steps=300 video=D2E728DEC150233C state=F891E7DC4E7F4001"
```

`--frame` writes the final framebuffer through the palette as a
`320×200` P6 PPM — upload it as a CI artefact to see what a hash
mismatch looks like. `tests/smoke.rs` pins the 90-step shell and
60-step galaxy goldens; regenerate after a deliberate behaviour change
with the commands in its doc header.
