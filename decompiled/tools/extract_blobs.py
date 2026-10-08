#!/usr/bin/env python3
"""Split each .BIN into per-record compressed streams (exact byte ranges)."""
import struct, sys, os, json
sys.path.insert(0, os.path.dirname(__file__))
from extract_png import lzss

def main():
    gamedir, outdir = sys.argv[1], sys.argv[2]
    manifest = []
    for fn in sorted(os.listdir(gamedir)):
        if not fn.endswith('.GPH'): continue
        s = fn[:-4]
        mode = s[:3]
        g = open(os.path.join(gamedir, fn), 'rb').read()
        data = open(os.path.join(gamedir, s + '.BIN'), 'rb').read()
        d = os.path.join(outdir, s); os.makedirs(d, exist_ok=True)
        n_streams = 4 if mode == 'EGA' else 1
        for n in range(len(g)//12):
            x, y, w, h, ofs, ex = struct.unpack('<4H2H', g[n*12:n*12+12])
            i = ofs*16; start = i
            total = 0
            for _ in range(n_streams):
                dec, i = lzss(data, i)
                i += 1 if mode == 'EGA' else 0
                total += len(dec)
            blob = data[start:i]
            open(os.path.join(d, f'{n:03}_{w}x{h}_x{x}y{y}{"_xor" if ex else ""}.bin'),
                 'wb').write(blob)
            manifest.append(dict(set=s, idx=n, x=x, y=y, w=w, h=h,
                                 ofs_para=ofs, ofs_byte=ofs*16, xor=ex,
                                 stream_len=len(blob), decoded=total))
    json.dump(manifest, open(os.path.join(outdir, 'manifest.json'), 'w'), indent=1)
    print('done', len(manifest))

main()
