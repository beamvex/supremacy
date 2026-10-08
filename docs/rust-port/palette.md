# `palette` — master MCGA DAC table

The 256×3 DAC table at dgroup `0x4994` (file `0x15034` in
`GAME.unpacked.exe`), programmed via `int 10/AX=1012`.

- `consts.rs` — `DAC_OFS = 0x15034`, `DAC_LEN = 768` (256 colours ×
  3 six-bit components).
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
