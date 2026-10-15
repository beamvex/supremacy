# `smoke` — headless CI mode (GAM-24)

## What this is for

GAM-9's "run N ticks deterministically and hash the framebuffer/audio
state so regressions are testable without a display". The
`supremacy-smoke` binary builds with default features — no `minifb`,
no `cpal` — so a CI runner with no windowing or audio device can compile
and run it. It steps the game a fixed number of frames, then prints one
line of hashes: if *any* behaviour changed — a pixel, a palette entry,
a PCM sample, a state byte, the RNG stream — one of the five numbers
changes and the field names which subsystem regressed.

The idea is the same as a snapshot test: you can't eyeball 60,000
pixels in CI, but you can hash them and compare the hash. Because the
hash covers five separate channels, a regression also tells you *where*
to look — `video` moved means the framebuffer, `state` moved means a
`ds:` cell, `audio` moved means the PCM stream.

## Running

```bash
cargo run --bin supremacy-smoke -- ../GAME --ticks 300
# smoke steps=300 video=D2E728DEC150233C pal=179DCB6EB754B64F
#       audio=F02A2B9E5641D61B state=F891E7DC4E7F4001 rng=00023B6B
```

Args: `<GAME-dir> [V] [K] [S]` positionally (the same PSP-tail letters
as `supremacy`/`supremacy-run` — see [args.md](args.md)), plus
`--set STEM`, `--exe PATH`, `--preset N`, `--seed N`, `--galaxy`,
`--ticks N`, `--script F.scr`, `--expect "k=v,…"`, `--frame F.ppm`.

The boot is `run.rs`'s: the image (`--exe` overrides the embedded
`rust/data/*.bin` blobs — GAM-55), one asset set, the mode
palette, `Game::new` over a `MemFs` (savegames never touch disk),
`set_assets` for `Call::Load`, `--galaxy` for `enter_galaxy` (see
[front.md](front.md)).

## The digest

`smoke::Digest` — five channels so a failure names its subsystem:

| field | content |
|---|---|
| `steps` | `Game::step`s run (wait-loop steps an op consumed count too) |
| `video` | FNV-1a of `Host::scr.buf` (mode-laid-out VRAM) |
| `pal` | FNV-1a of the 768 palette bytes |
| `audio` | FNV-1a of every rendered PCM `f32` — `SAMPLES_PER_STEP` (`BOOT_RATE`/30 = 1470) per step, the window loop's stride |
| `state` | FNV-1a of `difftest::ds_image` (the reassembled 64 KiB `ds`) |
| `rng` | the final `Rng` LCG state, raw `u32` |

FNV-1a is the tiny non-cryptographic hash used everywhere in the test
suite (`smoke::hash`): 64-bit, one multiply-xor fold per byte, fully
deterministic. It's not a checksum against tampering — it's a
"did this byte stream change" detector, which is all CI needs.

`state` is the interesting channel: it hashes *the reassembled `ds`
image*, not `State::block` alone — so a regression in the `low`/`hi`
regions ([game-vm.md](game-vm.md) § `Vm`) surfaces there too, not just
in saveable state.

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
See [difftest.md](difftest.md) § Script for the op table.

A scripted smoke run is the cheap version of a difftest: same input
language, but the oracle is a pinned hash instead of a DOS twin —
which is exactly the right tool when the original can't run in CI.

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

## Workflow — a failing smoke run

1. `supremacy-smoke … --expect …` fails on `video`: the framebuffer
   differs. Write `--frame out.ppm` and eyeball it — often the bug is
   obvious (a missing hotspot, a half-drawn panel).
2. `state` differs: the `ds` cells do. Compare a `difftest` `D01` dump
   at the same tick, or bisect `--ticks` to find the divergence frame.
3. `audio` differs but `video`/`state` don't: the PCM path — check
   `front::audio::Bus` and the driver's tick cadence.
4. `rng` differs alone: the LCG consumed a different number of draws —
   something called `rand`/`next16` more or fewer times than the asm.
   That's usually an ordering bug in a tick arm.

## File map

| file | item |
|---|---|
| `mod.rs` | module docs, re-exports |
| `digest.rs` | `Digest` — the five fields + `expect` matching |
| `hash.rs` | the FNV-1a helpers the channels share |
| `drive.rs` | the step loop — `Game::step` × N over `difftest::input` ops |

## Reading a real digest line

```
smoke steps=300 video=D2E7… pal=179D… audio=F02A… state=F891… rng=00023B6B
```

- `steps=300` — 300 `Game::step`s ran (wait-loop steps count too).
- `video` — FNV-1a over the mode-laid-out `Screen::buf`; changes when
  any drawn byte does.
- `pal` — the 768 DAC bytes; a palette regression (`present`/`shade`)
  moves this.
- `audio` — the rendered PCM `f32`s; a driver or `Bus` change moves it.
- `state` — the reassembled `ds` image; any `ds:` cell regression.
- `rng` — raw LCG state; moved alone means a draw-count ordering bug.
