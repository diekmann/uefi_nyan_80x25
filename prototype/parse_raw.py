#!/usr/bin/env python3


# gimp export as planar RGB. PLANAR!!!!
# gimp export as raw data just failed for me. I could not load the data back into gimp.
# also, sizes in bytes dod not work
# I tried C source code files, but also not obvious.
# so I picked HTML
# At least, I can open it in the browser and see what I got.

# first:
# cat OriginalNyan_40x25.html | grep BGCOLOR | sort | uniq
# only few colors! nice.

from collections import Counter
from pprint import pprint
import re

html = []

def parsecolor(line: str) -> str:
    m = re.match(rb'.*<TD +BGCOLOR=#(?P<rgb>[0-9a-f]*)>.*', line)
    if not m:
        raise Exception(f"did not parse: {line}")
    return m['rgb']


with open('OriginalNyan_40x25.html', 'rb') as f:
    while (line := f.readline()):
        if not b'BGCOLOR' in line:
            continue
        rgb = parsecolor(line)
        html.append(rgb)

colors = Counter(html)
print(f"there are {len(colors)} colors in the file.")
pprint(colors)

translate = {
    b'000000': 'BLK',
    b'ff99ff': 'LMG',
    b'999999': 'LGR',
    b'ffcc99': 'BWN',
    b'ff9999': 'LRD',
    b'ff3399': 'MGA',
    b'ffffff': 'WHT'
}

rust = []

for rgb in html:
    col = "BLK"
    try:
        col = translate[rgb]
    except KeyError as e:
        pass
    rust.append(col)

print(f"rust array has {len(rust)} entries, it sould have 80x25={80*25}")

print(", ".join(rust))