# `palette` — master MCGA DAC table + the runtime palette ops

The 256×3 DAC table at dgroup `0x4994` (file `0x15034` in
`GAME.unpacked.exe`), programmed via `int 10/AX=1012`. It is **not**
the palette the game runs on — screens compose the DAC at runtime:

- `Present(dx)` (`[0x125E]`/`0x2ED30`, `present.rs`) — stage row `dx`
  of the `ds:0x4394` table (16 × 32 colours, `ROWS_OFS = 0x14A34`)
  into regs `1..=31`, then `pal_adj`. `dx == 6` stages the whole
  `ds:0x4994` master into regs `1..=0xFF` instead.
- `pal_adj` (`0x2F1CF`, `adj.rs`) — DAC `0x21..=0xFF` from the
  `ds:0x4095` working table (`WORK_OFS = 0x14735`) colours `33..=255`.
- `pal_fade` (`[0x128E]`/`0x2F540`, `fade.rs`) — regs `0..=31` to black
  (the fade loop's end state), then `pal_adj`.
- `[0x1258]`/`[0x1260]` (`0x2EDBB`/`0x2EDA8`) — `int 10/AX=1012` over
  `CX=0x100` regs from `cs:0x1384`; the buffer ships zeroed (instant
  fade-to-black), handled as `Host::dac_flush`.
- Every staged nonzero component takes `+ds:[0x9CA1]` (the `al` add
  wraps), clamps at `0x3F`, then scales to 8-bit (`shade.rs`).

Verified against the DOS-side DAC dump `difftest/out/P05.BIN`: regs
`1..=31` match row `0xD` exactly, regs `33..` match `ds:0x4095`.

- `consts.rs` — `DAC_OFS = 0x15034`, `DAC_LEN = 768` (256 colours ×
  3 six-bit components), `WORK_OFS`, `ROWS_OFS`.
- `Palette` (`table.rs`) — `colors: [[u8; 3]; 256]` 8-bit RGB triples;
  `rgb(i)` (`rgb.rs`) returns one.
- `from_dac(raw)` (`from_dac.rs`) — builds a `Palette` from the raw
  768-byte table; each component scales `0..=63` → `0..=255` as
  `v * 255 / 63`, matching the reference renderer
  (`tools/extract_png.py`).
- `Palette::ega16()` / `Palette::cga()` (`modes.rs`) — the stock
  16-colour EGA/Tandy set (`EGA_PAL` in the reference tool) and CGA
  palette 1 high-intensity (black/cyan/magenta/white — the game's
  `int 10/AH=0B BH=1 BL=0` at file `0x31FE3`), used for non-MCGA modes.
