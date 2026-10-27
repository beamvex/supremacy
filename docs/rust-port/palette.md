# `palette` — master MCGA DAC table + the runtime palette ops

## What this module does

`palette` models the VGA palette the game runs on. It holds the shipped
**master DAC table** (256 colours, each a 6-bit RGB triple), and ports
the handful of runtime routines the game uses to *compose* the visible
palette — staging 32-colour UI rows, mixing in a working table,
fading to black, and applying a global shade offset.

## Background — VGA DAC registers

MCGA mode `0x13` shows 256 colours chosen from a palette: each of the
256 **DAC registers** holds one colour as three 6-bit components
(`0..=63` per channel; see [concepts.md](concepts.md) §7). Programs set
them with `int 10h/AX=1012` and a 768-byte table (256 × 3 bytes). The
port stores RGB as normal 8-bit components and scales with
`v * 255 / 63`, matching the reference renderer
(`decompiled/tools/extract_png.py`).

Why 6-bit? The VGA DAC is a physical 6-bit-per-channel converter —
`0x3F` is full intensity. The extra two bits of precision simply don't
exist in hardware, which is why the shipped tables are values `0..=63`
and why the port multiplies out (`255/63` = exactly 4.0476…) instead of
shifting.

### Three tables, one DAC

The game's colour data lives in three places, and the runtime ops are
just recipes for mixing them into the 256 live registers:

| table | `ds:` | file | contents |
|---|---|---|---|
| master | `0x4994` | `0x15034` | the canonical 256-colour set — the game's "full palette" |
| row table | `0x4394` | `0x14A34` | sixteen 32-colour rows — per-screen UI/widget colours |
| working | `0x4095` | `0x14735` | the 256-entry artwork table the current screen set up |

The master table ships in the exe at dgroup `0x4994` (file `0x15034`).
It is **not** the palette the game actually runs on — screens compose
the DAC at runtime: the row table supplies the low 32 registers (UI
chrome, drawn over whatever art is loaded) and the working table
supplies registers 33–255 (the scene's artwork colours). Register 0
(the border/background) and register 32 come out black — the asm's
staging buffers simply never write them.

- `Present(dx)` (`[0x125E]`/`0x2ED30`, `present.rs`, `stage.rs`) —
  stage row `dx` of the `ds:0x4394` table (16 × 32 colours,
  `ROWS_OFS = 0x14A34`) into regs `1..=31`, then `pal_adj`. `dx == 6`
  stages the whole `ds:0x4994` master into regs `1..=0xFF` instead —
  think of row 6 as "reset to the full master set" (and note it takes
  no `pal_adj` tail: the master *is* the whole palette).
- `pal_adj` (`0x2F1CF`, `adj.rs`) — DAC `0x21..=0xFF` from the
  `ds:0x4095` working table (`WORK_OFS = 0x14735`) colours `33..=255`:
  the second half of every compose — rows set the low UI registers,
  this fills in the artwork colours.
- `pal_fade` (`[0x128E]`/`0x2F540`, `fade.rs`) — regs `0..=31` to black
  (the fade loop's end state — the asm decrements components over
  vsync'd rounds, the port lands on the final state), then `pal_adj`.
- `[0x1258]`/`[0x1260]` (`0x2EDBB`/`0x2EDA8`) — `int 10/AX=1012` over
  `CX=0x100` regs from `cs:0x1384`; the buffer ships zeroed (instant
  fade-to-black), handled as `Host::dac_flush`.
- Every staged nonzero component takes `+ds:[0x9CA1]` (the `al` add
  wraps), clamps at `0x3F`, then scales to 8-bit (`shade.rs`) — a
  per-screen tint cell the game bumps to darken/lighten scenes. The
  `gate` flag is the subtlety: the row/`pal_adj` loops leave *zero*
  components at zero (the asm's `cmp al,0` skip — so black stays pure
  black and is never tinted), while the `dx == 6` master path adds
  unconditionally.

Verified against a DAC dump captured from the running original: regs
`1..=31` match row `0xD` exactly, regs `33..` match `ds:0x4095`.

## API tour

- `consts.rs` — `DAC_OFS = 0x15034` (file offset of the raw table),
  `DAC_LEN = 768` (256 colours × 3 six-bit components), `WORK_OFS`,
  `ROWS_OFS`.
- `Palette` (`table.rs`) — `colors: [[u8; 3]; 256]` 8-bit RGB triples;
  `rgb(i)` (`rgb.rs`) returns one — that is what `Screen::pixels()`
  indexes (see [video.md](video.md)). A `Palette` in this crate is
  "the 256 registers as the host would show them", not the raw 6-bit
  source — `from_dac` and the stage ops produce it already scaled.
- `from_dac(raw) -> Palette` (`from_dac.rs`) — builds a `Palette` from
  the raw 768-byte table, scaling `0..=63` → `0..=255`.
- `Palette::ega16()` / `Palette::cga()` (`modes.rs`) — the stock
  16-colour EGA/Tandy set (`EGA_PAL` in the reference tool — the classic
  `0x00`/`0x55`/`0xAA`/`0xFF` 64-colour cube) and CGA
  palette 1 high-intensity (black/cyan/magenta/white — the game's
  `int 10/AH=0B BH=1 BL=0` at file `0x31FE3`), used for non-MCGA modes.
  In these modes the DAC is hardware-fixed, so the palette *is* a
  constant rather than a runtime composition.

## Example

```rust
let img = supremacy::img::image();    // the embedded image (rust/data/*.bin)
let pal = supremacy::palette::from_dac(
    &img[supremacy::palette::DAC_OFS..supremacy::palette::DAC_OFS + 768],
);
let [r, g, b] = pal.rgb(17);          // 8-bit RGB of palette index 17
```

The MCGA path in `bin/supremacy.rs` does exactly this; EGA/TGA fall back
to `Palette::ega16()` and CGA to `Palette::cga()` since their palettes
are fixed hardware sets, not a 256-entry DAC table.

## Pitfalls

- **6-bit source, 8-bit consumer.** The shipped tables are `0..=63`;
  `Palette::colors` is `0..=255`. `from_dac` does the scaling — don't
  index raw table bytes expecting full range.
- **The live palette ≠ the master table.** Master is a *reset source*;
  what a screen shows is a row + working-table composition through the
  shade cell. Comparing rendered pixels against `from_dac` alone will
  mismatch whenever a row staged different colours.
- **`gate` keeps zero black.** Shade arithmetic skips zero components
  in the row/working paths — a naive `saturating_add` would tint blacks.
- **EGA/CGA/TGA don't use this table at all.** Their palettes are
  hardware-fixed (`ega16`/`cga`); only MCGA programs the DAC.

## File map

| file | item |
|---|---|
| `mod.rs` | module docs, re-exports |
| `consts.rs` | `DAC_OFS`, `DAC_LEN`, `WORK_OFS`, `ROWS_OFS` |
| `table.rs` | `Palette { colors }` |
| `rgb.rs` | `Palette::rgb(i) -> [u8;3]` |
| `from_dac.rs` | `from_dac` — raw 6-bit table → `Palette` |
| `modes.rs` | `ega16()`, `cga()` fixed palettes |
| `shade.rs` | the `+ds:[0x9CA1]`, clamp, scale component rule |
| `stage.rs` | `Palette::stage` — the shared range-load loop |
| `present.rs` | `Present(dx)` — `[0x125E]` row/master staging |
| `adj.rs` | `pal_adj` — working table → regs `0x21..=0xFF` |
| `fade.rs` | `pal_fade` — regs `0..=31` → black + `pal_adj` |

## Worked example — one staged register

`Present(0xD)` (the title screen's row) stages `ds:0x4394 + 0xD*0x60`
into regs 1..=31. Say the row's colour 5 is `34 12 08` (6-bit), and
`ds:[0x9CA1] = 4` (a darkened scene): each component takes `+4`,
clamps `0x3F`, scales `*255/63` — `0x38*255/63 = 0xE0`, `0x16*255/63 =
0x5B`, `0x0C*255/63 = 0x30` — so `pal.rgb(5)` reads `[0xE0, 0x5B,
0x30]`. A zero component stays zero even with the shade (the `gate`),
so pure blacks never tint.
