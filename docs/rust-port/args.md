# `args` — command-line parser

## What this module does

The original game is launched as `GAME <V> <K> <S>` — three
single-letter arguments picking the video mode, the mouse mode, and the
sound driver (e.g. `SUPRE.BAT` runs `GAME M M A` = MCGA video + real
mouse + AdLib). `args` ports the routine that reads those letters out of
the DOS PSP command tail and turns them into a `Config` — the same
values the asm writes into its configuration cells.

## Background — the PSP command tail

When DOS launches a program it places a Program Segment Prefix in front
of it and copies the typed tail to `es:0x80` (see
[concepts.md](concepts.md) §4). The game never scans that string — it
reads three **fixed cells** instead: `es:0x82` = video letter,
`es:0x84` = keyboard flag, `es:0x86` = sound letter. So the input to the
parser is the raw tail bytes: count byte at `0x80`, a space at `0x81`,
then the letters at `0x82`/`0x84`/`0x86` — one letter every *other*
byte, with the gaps ignored (a missing cell reads `0` and lands on the
asm's fall-through defaults, which is why bare `GAME` boots MCGA + PC
speaker).

Port of `0x31FF8`–`0x3207D`.

## What the asm writes

`Config` (`config.rs`) mirrors the four cells the parser sets in the
original: `[0x9CC6]` (video mode number), `[0x9CCD]` (keyboard-mouse
flag), `[0x155]` (sound driver index) and `[0x156]` (driver-variant
paragraph selector that picks which int-8 handler the PIT hook
installs).

## API tour

`parse(tail) -> Config` (`parse.rs`) takes the bytes from `es:0x80`
(count byte, space, letters every other byte):

```rust
// Tail for "GAME M M A": count, space, 'M', ' ', 'M', ' ', 'A'.
let tail = [6u8, b' ', b'M', b' ', b'M', b' ', b'A'];
let cfg = supremacy::args::parse(&tail);
assert!(matches!(cfg.video, supremacy::args::Video::Mcga));
assert!(!cfg.keyboard_mouse);          // 'M' = real mouse, not 'K'
assert!(matches!(cfg.sound, supremacy::args::Sound::AdLib));
```

`Config`'s fields:

- `video: Video` — `Video::from_arg` (`video_from.rs`): `C`/`T`/`E`/`M`
  → `Cga`/`Tga`/`Ega`/`Mcga`; anything else falls through into the MCGA
  arm (index 3), which is why bare `GAME` gets MCGA.
- `keyboard_mouse: bool` — `es:0x84 == 'K'`: hook int 33h and emulate
  the mouse from the keyboard (`[0x9CCD] = 1`); any other value = real
  mouse. `SUPRE.BAT`'s `M` means "real mouse"; `K` is the accessibility
  mode whose arrow keys move the cursor (see [platform.md](platform.md)
  `nav_dir`/`feed_scan`).
- `sound: Sound` + `timer_variant: u16` — `Sound::from_arg`
  (`sound_from.rs`) ports the compare chain at `0x32042`–`0x32073`,
  returning the `[0x155]` index and the `[0x156]` selector that picks
  which int-8 handler the PIT hook installs:

| letter | `Sound` | `[0x155]` | `[0x156]` timer variant |
|---|---|---|---|
| `T` | `Tandy` | 2 | `0xD5` |
| `A` | `AdLib` | 3 | `0x1D0` |
| `R` | `Roland` | 4 | `0x1D0` |
| other | `PcSpeaker` | 1 | `0` |

The `[0x155]` number is the same index the `int 0x80` sound interface
dispatches on (see [audio.md](audio.md)); `[0x156]` is the paragraph
offset of the driver module's int-8 timer install — the P/T engines run
their own trackers at the PIT rate, the A/R module installs its own
service routine instead.

## `Video` helpers

- `stem()` (`video_stem.rs`) — the asset-set stem patched into the
  `???.???` filename templates at `ds:0x1208+`: `CGA`/`TGA`/`EGA`/`MCG`.
  The loaded asset set is `stem + ".BIN"` + `stem + ".GPH"`
  (`MCG.BIN`/`MCG.GPH` for MCGA).
- `template_seg()` (`video_template.rs`) — dgroup offset of the mode's
  `0x115B`-byte parameter template (`0xD3A2`/`0xE4FD`/`0xC247`/`0xB0EC`),
  copied to `ds:0x143` by the `rep movsb` at `0x32034`. That copy is
  where every per-mode constant the rest of the game reads (VRAM
  segment, dispatch table, font pointers) comes from — see
  [video.md](video.md) and `decompiled/FUNCTIONS.md` § "Mode parameter
  templates".

## How the binaries use it

The shipped binaries take the letters as ordinary argv strings and
rebuild the PSP tail so `parse` sees exactly the bytes the asm sees
(`tail_bytes` in `bin/supremacy.rs`): count byte, space, then the
letters at the `0x82`/`0x84`/`0x86` positions. `GAME M M A` and
`supremacy GAME M ' ' A` therefore produce identical `Config`s — the
keyboard arg is a space when a real mouse is wanted.
