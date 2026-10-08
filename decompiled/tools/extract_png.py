#!/usr/bin/env python3
"""Extract Supremacy .BIN/.GPH image sets to PNG (pure stdlib).

Codec: LZSS N=4096 F=18 THRESH=2, ring zero-init, r=0xFEE,
flag bits LSB-first (1=literal, 0=match word token
{pos&0xFF, (pos>>4)&0xF0 | (len-3)}), terminator: match flag +
00 00 followed by byte 0.

Modes:  MCG = 8bpp packed        w bytes/row
        TGA = 4bpp packed        2 px/byte, hi nibble first
        CGA = 2bpp packed        4 px/byte, MSB pair first
        EGA = 4bpp planar        4 consecutive LZSS streams (1 bitplane each)
"""
import struct, sys, os, zlib, json

DG = 0xFAA*16 + 0xC00
EXE = os.path.join(os.path.dirname(__file__), '..', 'GAME.unpacked.exe')

def lzss(buf, i=0):
    """Decode one LZSS stream from buf[i:]; returns (data, consumed)."""
    ring = bytearray(4096); r = 0xFEE; out = bytearray(); flags = 0
    while i < len(buf):
        flags >>= 1
        if not (flags & 0x100):
            flags = buf[i] | 0xFF00; i += 1
        if flags & 1:
            b = buf[i]; i += 1
            out.append(b); ring[r] = b; r = (r+1) & 0xFFF
        else:
            if i+1 >= len(buf): break
            t = buf[i] | (buf[i+1] << 8); i += 2
            if t == 0 and (i >= len(buf) or buf[i] == 0): break
            pos = (t & 0xFF) | ((t & 0xF000) >> 4); ln = ((t >> 8) & 0xF) + 3
            for _ in range(ln):
                b = ring[pos]; out.append(b)
                ring[r] = b; r = (r+1) & 0xFFF; pos = (pos+1) & 0xFFF
    return bytes(out), i

def png(path, w, h, idx, pal):
    raw = b''.join(b'\x00' + bytes(idx[y*w:(y+1)*w]) for y in range(h))
    def chunk(t, d):
        c = struct.pack('>I', len(d)) + t + d
        return c + struct.pack('>I', zlib.crc32(t + d) & 0xFFFFFFFF)
    pl = b''.join(bytes(min(255, p) for p in px) for px in pal)
    data = (b'\x89PNG\r\n\x1a\n'
        + chunk(b'IHDR', struct.pack('>IIBBBBB', w, h, 8, 3, 0, 0, 0))
        + chunk(b'PLTE', pl)
        + chunk(b'IDAT', zlib.compress(raw))
        + chunk(b'IEND', b''))
    open(path, 'wb').write(data)

def load_palette():
    exe = open(EXE, 'rb').read()
    off = DG + 0x4994   # master 256-colour DAC table in dgroup
    return [tuple(c*255//63 for c in exe[off+i*3:off+i*3+3]) for i in range(256)]

EGA_PAL = [(0,0,0),(0,0,170),(0,170,0),(0,170,170),(170,0,0),(170,0,170),
           (170,85,0),(170,170,170),(85,85,85),(85,85,255),(85,255,85),
           (85,255,255),(255,85,85),(255,85,255),(255,255,85),(255,255,255)]
CGA_PAL = [(0,0,0),(85,255,255),(255,85,255),(255,255,255)]   # palette 1 hi

def expand(data, w, h, mode):
    """Return (pixels, pix_w) — one index per pixel, row-major."""
    if mode == 'MCG':
        return list(data), w
    if mode == 'TGA':           # 4bpp packed
        out = []
        for b in data:
            out += [b >> 4, b & 0xF]
        return out, w*2
    if mode == 'CGA':           # 2bpp packed
        out = []
        for b in data:
            out += [(b >> 6) & 3, (b >> 4) & 3, (b >> 2) & 3, b & 3]
        return out, w*4
    if mode == 'EGA':           # data = 4 planes of w*h bytes
        out = bytearray(w*8*h)
        pw = w*8
        plane_sz = w*h
        for p in range(4):
            plane = data[p*plane_sz:(p+1)*plane_sz]
            for row in range(h):
                for byte_i in range(w):
                    b = plane[row*w + byte_i]
                    for bit in range(8):
                        if b & (0x80 >> bit):
                            out[row*pw + byte_i*8 + bit] |= (1 << p)
        return list(out), pw

def decode_rec(data, ofs, w, h, mode):
    n_streams = 4 if mode == 'EGA' else 1
    i = ofs*16
    chunks = []
    for _ in range(n_streams):
        dec, i = lzss(data, i)
        i += 1 if mode == 'EGA' else 0   # EGA decoder eats trailing 0x00
        chunks.append(dec)
    return b''.join(chunks)

def main():
    gamedir, outdir = sys.argv[1], sys.argv[2]
    if len(sys.argv) > 3:
        sets = sys.argv[3].split(',')
    else:
        sets = ['MCG','MCGIN','MCGDTH','MCGWIN','CGA','CGAIN','CGADTH','CGAWIN',
                'EGA','EGAIN','EGADTH','EGAWIN','TGA','TGAIN','TGADTH','TGAWIN']
    vga_pal = load_palette()
    manifest = []
    bad = 0
    for s in sets:
        mode = s[:3]
        pal = vga_pal if mode == 'MCG' else (CGA_PAL if mode == 'CGA' else EGA_PAL)
        g = open(os.path.join(gamedir, s + '.GPH'), 'rb').read()
        data = open(os.path.join(gamedir, s + '.BIN'), 'rb').read()
        d = os.path.join(outdir, s); os.makedirs(d, exist_ok=True)
        for n in range(len(g)//12):
            x, y, w, h, ofs, ex = struct.unpack('<4H2H', g[n*12:n*12+12])
            dec = decode_rec(data, ofs, w, h, mode)
            exp = w*h*(4 if mode == 'EGA' else 1)
            ok = len(dec) == exp
            if not ok:
                bad += 1
                print(f'{s}/{n}: got {len(dec)} want {exp}', file=sys.stderr)
                dec = (dec + b'\0'*exp)[:exp]
            pix, pw = expand(dec, w, h, mode)
            png(os.path.join(d, f'{n:03}_{pw}x{h}_x{x}y{y}{"_xor" if ex else ""}.png'),
                pw, h, pix, pal)
            manifest.append(dict(set=s, idx=n, x=x, y=y, w=w, h=h, pw=pw,
                                 ofs=ofs, xor=ex, decoded=len(dec), ok=ok))
    json.dump(manifest, open(os.path.join(outdir, 'manifest.json'), 'w'), indent=1)
    print('done', len(manifest), 'images;', bad, 'size mismatches')

if __name__ == '__main__':
    main()
