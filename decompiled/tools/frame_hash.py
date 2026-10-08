#!/usr/bin/env python3
"""Generate rust/tests/fixtures/frames.tsv — framebuffer hashes for the
video::Screen draw_image port.

Two independent checks per case:
  buf_fnv — FNV-1a of the raw VRAM buffer produced by a Python model of the
            per-mode draw routines (di computation + banked row advance,
            FUNCTIONS.md), NOT the Rust code.
  px_fnv  — FNV-1a of a 320x200 pixel canvas composited from the verified
            reference PNGs in decompiled/png/<SET>/ (opaque -> copy,
            xor -> pixel xor). Verifies Screen::pixels() against the
            GAM-2-verified renderer rather than the same model.

Cases draw records in order, exactly like Screen::draw_image calls.
"""
import os, struct, zlib

HERE = os.path.dirname(__file__)
ROOT = os.path.join(HERE, '..', '..')
GAME = os.path.join(ROOT, 'GAME')
PNG = os.path.join(ROOT, 'decompiled', 'png')
OUT = os.path.join(ROOT, 'rust', 'tests', 'fixtures', 'frames.tsv')

def lzss_decode(buf, i=0):
    """LZSS from extract_png.py — returns (decoded, index past terminator)."""
    ring = bytearray(4096)
    r, out, flags = 0xFEE, bytearray(), 0
    while i < len(buf):
        flags >>= 1
        if not (flags & 0x100):
            flags = buf[i] | 0xFF00
            i += 1
        if flags & 1:
            b = buf[i]
            i += 1
            out.append(b)
            ring[r] = b
            r = (r + 1) & 0xFFF
        else:
            if i + 1 >= len(buf):
                break
            t = buf[i] | (buf[i + 1] << 8)
            i += 2
            if t == 0 and (i >= len(buf) or buf[i] == 0):
                break
            pos = (t & 0xFF) | ((t & 0xF000) >> 4)
            ln = ((t >> 8) & 0xF) + 3
            for _ in range(ln):
                b = ring[pos]
                out.append(b)
                ring[r] = b
                r = (r + 1) & 0xFFF
                pos = (pos + 1) & 0xFFF
    return bytes(out), i


W, H = 320, 200
GEOM = {  # mode -> (buf_len, span, pitch, px_per_byte)
    'MCG': (0xFA00, None, None, 1),
    'EGA': (4 * 0x1F40, None, None, 8),
    'CGA': (0x4000, 0x4000, 0x50, 4),
    'TGA': (0x8000, 0x8000, 0xA0, 2),
}


def fnv(b):
    h = 0xcbf29ce484222325
    for x in b:
        h = ((h ^ x) * 0x100000001b3) & 0xFFFFFFFFFFFFFFFF
    return h


def records(set_name):
    g = open(os.path.join(GAME, set_name + '.GPH'), 'rb').read()
    return [struct.unpack('<6H', g[i:i + 12]) for i in range(0, len(g) - 11, 12)]


def di_at(mode, x, y):
    if mode == 'CGA':
        return (y >> 1) * 0x50 + (y & 1) * 0x2000 + x
    if mode == 'TGA':
        return (y >> 2) * 0xA0 + (y & 3) * 0x2000 + x
    if mode == 'EGA':
        return y * 0x28 + x
    return y * 0x140 + x


def emit(buf, mode, di, w, xor, data):
    """Feed decoded bytes through the mode's row-advance emit model."""
    row_left = w
    for b in data:
        if xor:
            buf[di] ^= b
        else:
            buf[di] = b
        di += 1
        row_left -= 1
        if row_left == 0:
            row_left = w
            di = advance(mode, di, w)


def advance(mode, di, w):
    if mode == 'MCG':
        return di - w + 0x140
    if mode == 'EGA':
        return di - w + 0x28
    di += 0x2000 - w
    if di >= GEOM[mode][1]:
        di += GEOM[mode][2] - GEOM[mode][1]
    return di


def draw(buf, mode, bin_, rec):
    x, y, w, h, ofs, flag = rec
    if flag not in (0, 1):
        return 0
    xor = flag == 1
    i = ofs * 16
    n_planes = 4 if mode == 'EGA' else 1
    for p in range(n_planes):
        data, i = lzss_decode(bin_, i)
        if mode == 'EGA':
            i += 1  # EGA decoder eats the terminator's trailing byte
        plane = buf if mode != 'EGA' else memoryview(buf)[p * 0x1F40:(p + 1) * 0x1F40]
        emit(plane, mode, di_at(mode, x, y), w, xor, data)


def png_pixels(set_name, idx, x, y, w, h, xor, pw):
    name = f'{idx:03}_{pw}x{h}_x{x}y{y}{"_xor" if xor else ""}.png'
    d = open(os.path.join(PNG, set_name, name), 'rb').read()
    pos, idat = 8, b''
    while pos < len(d):
        ln, typ = struct.unpack('>I4s', d[pos:pos + 8])
        if typ == b'IDAT':
            idat += d[pos + 8:pos + 8 + ln]
        pos += 12 + ln
    raw = zlib.decompress(idat)
    rows, stride = [], pw + 1
    for r in range(h):
        row = raw[r * stride:(r + 1) * stride]
        assert row[0] == 0, 'unexpected PNG filter'
        rows.append(row[1:])
    return rows


def px_hash(set_name, mode, idxs):
    """Composite the reference PNGs into a canvas like Screen::pixels()."""
    canvas = bytearray(W * H)
    ppb = GEOM[mode][3]
    for i, rec in enumerate(records(set_name)):
        if i not in idxs:
            continue
        x, y, w, h, _, flag = rec
        rows = png_pixels(set_name, i, x, y, w, h, flag == 1, w * ppb)
        for r, row in enumerate(rows):
            for c, px in enumerate(row):
                o = (y + r) * W + x * ppb + c
                canvas[o] = canvas[o] ^ px if flag == 1 else px
    return fnv(canvas)


def case(set_name, idxs):
    mode = set_name[:3]
    bin_ = open(os.path.join(GAME, set_name + '.BIN'), 'rb').read()
    buf = bytearray(GEOM[mode][0])
    recs = records(set_name)
    for i in idxs:
        draw(buf, mode, bin_, recs[i])
    return mode, buf


def main():
    lines = []
    for set_name in ('MCG', 'EGA', 'CGA', 'TGA'):
        for lo in (0, 10, 20, 30):
            idxs = list(range(lo, lo + 10))
            mode, buf = case(set_name, idxs)
            name = f'{set_name}_{lo}_{lo+9}'
            lines.append(f'{name}\t{set_name}\t{lo}-{lo+9}\t'
                         f'{len(buf)}\t{fnv(buf):016x}\t{px_hash(set_name, mode, set(idxs)):016x}')
    open(OUT, 'w').write('\n'.join(lines) + '\n')
    print('\n'.join(lines))


if __name__ == '__main__':
    main()
