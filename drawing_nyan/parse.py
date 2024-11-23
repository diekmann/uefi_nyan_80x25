#!/usr/bin/env python3

import re

def parsecolor(line: str) -> str:
    m = re.match(r'.*<TD +BGCOLOR=#(?P<rgb>[0-9a-f]*)>.*', line)
    if not m:
        raise Exception(f"did not parse: {line}")
    return m['rgb']

assert(parsecolor('<TD  BGCOLOR=#aabbcc>') == 'aabbcc')

with open('img/nyan.html', 'r') as f:
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

print(", ".join([translate[rgb] for rgb in html]))