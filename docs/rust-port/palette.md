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

The master table ships in the exe at dgroup `0x4994` (file `0x15034`).
It is **not** the palette the game actually runs on — screens compose
the DAC at runtime:

- `Present(dx)` (`[0x125E]`/`0x2ED30`, `present.rs`) — stage row `dx`
  of the `ds:0x4394` table (16 × 32 colours, `ROWS_OFS = 0x14A34`)
  into regs `1..=31`, then `pal_adj`. `dx == 6` stages the whole
  `ds:0x4994` master into regs `1..=0xFF` instead — think of row 6 as
  "reset to the full master set".
- `pal_adj` (`0x2F1CF`, `adj.rs`) — DAC `0x21..=0xFF` from the
  `ds:0x4095` working table (`WORK_OFS = 0x14735`) colours `33..=255`:
  the second half of every compose — rows set the low UI registers,
  this fills in the artwork colours.
- `pal_fade` (`[0x128E]`/`0x2F540`, `fade.rs`) — regs `0..=31` to black
  (the fade loop's end state), then `pal_adj`.
- `[0x1258]`/`[0x1260]` (`0x2EDBB`/`0x2EDA8`) — `int 10/AX=1012` over
  `CX=0x100` regs from `cs:0x1384`; the buffer ships zeroed (instant
  fade-to-black), handled as `Host::dac_flush`.
- Every staged nonzero component takes `+ds:[0x9CA1]` (the `al` add
  wraps), clamps at `0x3F`, then scales to 8-bit (`shade.rs`) — a
  per-screen tint cell the game bumps to darken/lighten scenes.

Verified against the DOS-side DAC dump `difftest/out/P05.BIN`: regs
`1..=31` match row `0xD` exactly, regs `33..` match `ds:0x4095`.

## API tour

- `consts.rs` — `DAC_OFS = 0x15034` (file offset of the raw table),
  `DAC_LEN = 768` (256 colours × 3 six-bit components), `WORK_OFS`,
  `ROWS_OFS`.
- `Palette` (`table.rs`) — `colors: [[u8; 3]; 256]` 8-bit RGB triples;
  `rgb(i)` (`rgb.rs`) returns one — that is what `Screen::pixels()`
  indexes (see [video.md](video.md)).
- `from_dac(raw) -> Palette` (`from_dac.rs`) — builds a `Palette` from
  the raw 768-byte table, scaling `0..=63` → `0..=255`.
- `Palette::ega16()` / `Palette::cga()` (`modes.rs`) — the stock
  16-colour EGA/Tandy set (`EGA_PAL` in the reference tool) and CGA
  palette 1 high-intensity (black/cyan/magenta/white — the game's
  `int 10/AH=0B BH=1 BL=0` at file `0x31FE3`), used for non-MCGA modes.

## Example

```rust
let img = std::fs::read("decompiled/GAME.unpacked.exe")?;
let pal = supremacy::palette::from_dac(
    &img[supremacy::palette::DAC_OFS..supremacy::palette::DAC_OFS + 768],
);
let [r, g, b] = pal.rgb(17);          // 8-bit RGB of palette index 17
```

The MCGA path in `bin/supremacy.rs` does exactly this; EGA/TGA fall back
to `Palette::ega16()` and CGA to `Palette::cga()` since their palettes
are fixed hardware sets, not a 256-entry DAC table.
