# `args` — command-line parser

Port of `0x31FF8`–`0x3207D`: `GAME <C/T/E/M> <K> <P/T/A/R>` reads
single-letter arguments from fixed cells in the PSP command tail —
`es:0x82` = video, `es:0x84` = keyboard flag, `es:0x86` = sound.

`parse(tail)` (`parse.rs`) takes the bytes from `es:0x80` (count byte,
space, letters every other byte); a missing cell reads `0` and lands on
the asm's fall-through defaults.

`Config` (`config.rs`) — what the asm writes to `[0x9CC6]`, `[0x9CCD]`,
`[0x155]`, `[0x156]`:

- `video: Video` — `from_arg` (`video_from.rs`): `C`/`T`/`E`/`M` →
  `Cga`/`Tga`/`Ega`/`Mcga`; anything else falls through into the MCGA arm
  (index 3), which is why bare `GAME` gets MCGA.
- `keyboard_mouse: bool` — `es:0x84 == 'K'`: hook int 33h and emulate the
  mouse from the keyboard (`[0x9CCD] = 1`); any other value = real mouse.
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

`Video` helpers:

- `stem()` (`video_stem.rs`) — the asset-set stem patched into the
  `???.???` filename templates at `ds:0x1208+`: `CGA`/`TGA`/`EGA`/`MCG`.
- `template_seg()` (`video_template.rs`) — dgroup offset of the mode's
  `0x115B`-byte parameter template (`0xD3A2`/`0xE4FD`/`0xC247`/`0xB0EC`),
  copied to `ds:0x143` by the `rep movsb` at `0x32034`.

`SUPRE.BAT`'s `GAME M M A` = MCGA + real mouse + AdLib; a second arg of
`K` instead of `M` selects keyboard-emulated mouse.
