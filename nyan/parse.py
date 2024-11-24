#!/usr/bin/env python3

import re
import sys

def parsecolor(line: str) -> str:
    m = re.match(r'.*<TD +BGCOLOR=#(?P<rgb>[0-9a-f]*)>.*', line)
    if not m:
        raise Exception(f"did not parse: {line}")
    return m['rgb']

assert(parsecolor('<TD  BGCOLOR=#aabbcc>') == 'aabbcc')

with open(sys.argv[1], 'r') as f:
    lines = f.readlines()

html = [parsecolor(line) for line in lines if 'BGCOLOR' in line]

translate = {
    '000000': 'Black',
    '0200ff': 'Blue',
    'ff99ff': 'LightMagenta',
    '999999': 'LightGray',
    'ffcc99': 'Brown',
    'ff9999': 'LightRed',
    'ff3399': 'Magenta',
    'ffffff': 'White'
}

frame_num = sys.argv[1][len('frame'):-len('.html')]
print(f'pub const NYAN_80X25_{frame_num}: [Color; 80*25] = [' + ', '.join([translate[rgb] for rgb in html]) + '];\n')