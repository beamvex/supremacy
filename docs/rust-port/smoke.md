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
| `state` | FNV-1a of `snap::ds_image` (the reassembled 64 KiB `ds`) |
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

`--script` takes the `.scr` text format — one op per line, all fields
hex (a leading `0x` is tolerated; `#` starts a comment):

```
0A  key 1C            # Enter make at tick 0x0A
0E  key 9C            # break
14  mouse 1E 160 96 1 # event mask, x, y, buttons
28  waitcell 91CD 5   # hold until ds:0x91CD == 5 (uimode == galaxy)
32  pokew drv 55A2 1234
1E  dump ds 7D39 1F8B 01
1E  pal 05
1E  stat 06
3C  done
```

| op | fields | effect |
|---|---|---|
| `key` | scancode (bit 7 = break) | `[0x9CCB]`/`[0x9CCC]` latch + `drv:0x8B72` LIFO push — `Pump::key` |
| `mouse` | mask, x, y, buttons | moves `Pump`'s cursor + button state like the int-33h handler |
| `pokew` | seg, ofs, word | word write at `ds:`/`drv:` offset |
| `waitcell` | ds ofs, byte | steps until `ds:[ofs] == val` (state sync point) |
| `dump` | seg, ofs, len, id | no-op — snapshots here are the hash channels |
| `pal` | id | no-op |
| `stat` | id | no-op |
| `done` | — | ends the run at its tick |

`seg` is `ds` (game dgroup), `drv` (the driver/template segment —
holds the `0x8B72` key LIFO and the `0x55A2` LCG state), or an absolute
hex segment (`A000` = VRAM).

The op semantics live in `smoke::input`: `key` waits for the
`[0x9CCB]` latch to free first (bounded at 5000 internal steps),
`pokew` mirrors `drv:` writes into `Host::img` at `0x292B0 + ofs`. The
oracle is a pinned hash — which is exactly the right tool for CI.

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
2. `state` differs: the `ds` cells do. Bisect `--ticks` to find the
   divergence frame, then diff the `ds` image against the same run on
   the last-good commit.
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
| `drive.rs` | the step loop — `Game::step` × N over the script ops |
| `op.rs` | `Op`/`Rec` — the `.scr` op vocabulary |
| `script.rs` | the `.scr` text parser → `Rec` list |
| `seg.rs` | `ds`/`drv`/absolute segment selectors |
| `input.rs` | `key`/`mouse`/`pokew`/`waitcell` application |
| `snap.rs` | `ds_image` — the reassembled 64 KiB `ds` snapshot |

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
