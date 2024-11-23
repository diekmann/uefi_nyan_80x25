# Nyan Cat to Rust Array

It's time to convert nyan cat into 80x25 UEFI UCS-2 `BLOCKELEMENT_FULL_BLOCK`s.

## Getting Nyan Cat

Here is nyan cat:

![Original Nyan Cat by prguitarman. All rights reserved.](../nyan_img/original.gif)

It's probably copyrighted, so if someone asks, we pretend an AI drew that image for us.
Just kidding.
It's the original nyan cat by prguitarman from https://nyan.cat/.
Huge thanks to Chris Torres for allowing me to use it for this project! :heart:

## Downsizing Nyan Cat

Let's downsize that image to 80x25.
Here it is opened in Gimp.

![nyan cat opened in Gimp](img/nyan_gimp.png)

The first thing I notice is that it has many legs and tails.
That's because it's an animated gif and Gimp shows all frames simultaneously.
See the panel on the bottom right.

Let's limit ourselves to the first frame only for now.

![nyan cat opened in Gimp, only the first frame](img/nyan_gimp1.png)

Let's downsize it.
It currently is 272x168 pixels high.

![Gimp downsizing dialog](img/downsize_cubic.png)

To preserve the aspect ration, the largest we can go is 40x25 pixels.

![Downsized Nyan cat to 40x25 pixels with cubic interpolations](img/downsized_cubic.png)

That doesn't look good!
When zooming in, we se transparent pixels and artifacts that look like anti aliasing.
First of all, that just doesn't look good at that tiny size.
Second, nyan cat is pixel art, anti aliasing doesn't belong here.
Third, the UEFI simple text output protocol wouldn't support those colors anyway.

Let's try again.
In the above "Scale Image" dialog, we stuck with the defaults.
See the interpolation "Cubic" in there?
We don't want interpolation.
We want pixel perfect downsizing.

Let's try again, but this time, without interpolation.

![Gimp downsizing dialog, selecting interpolation "none"](img/downsize_none.png)

Purrrfect! :cat:

![Pixel perfect downsized Nyan cat to 40x25](img/downsized_none.png)

## Nyan to Vec

### Exporting Nyan Cat

We want to turn this picture into a Rust vector of UEFI colors.
So we somehow want to export this into an uncompressed binary list of colors.

Surely, exporting the image as raw image data in RGB is what we are looking for.

![Gimp export as raw image data](img/export_raw.png)

I need to be honest: I tried parsing and understanding the resulting `.data` file for quite some time.
But I failed.
I could not figure out how to parse this data.
When loading back the image into Gimp, I need to manually specify the width and the size.
So I assume, there is no header with metadata.
Also, the beginning of the file looks like it just starts with the raw transparent pixels:

```bash
$ xxd original.data | head
00000000: 0000 0000 0000 0000 0000 0000 0000 0000  ................
00000010: 0000 0000 0000 0000 0000 0000 0000 0000  ................
00000020: 0000 0000 0000 0000 0000 0000 0000 0000  ................
00000030: 0000 0000 0000 0000 0000 0000 0000 0000  ................
00000040: 0000 0000 0000 0000 0000 0000 0000 0000  ................
00000050: 0000 0000 0000 0000 0000 0000 0000 0000  ................
00000060: 0000 0000 0000 00ff 00ff 00ff 00ff 00ff  ................
00000070: 00ff 00ff 00ff 00ff 00ff 00ff 00ff 00ff  ................
00000080: 00ff 00ff 00ff 00ff 00ff 00ff 00ff 0000  ................
00000090: 0000 0000 0000 0000 0000 0000 0000 0000  ................
```

Long story short: I could not figure out how to parse this format.
The file is exactly 2000 bytes long.
If I understood the export dialog correctly, the format should be RGB, with at least one byte per color value.
But 2000 isn't divisible by 3.
In addition, the image is 40x25 in size.
But 2000 isn't divisible by 25 as well.

I don't understand the format.
But surely Gimp does, since it created it, right?

Specifying the correct size during the import, this is what I got back:

![unrecognizable pixels](img/open_data_in_gimp.png)

Okay, if even Gimp cannot understand its own file format, maybe something is truly wrong.
If you know what happened here, please let me know.

I didn't find information in the Gimp docs about this data format.
So it's time to read the Gimp sources, right?
Well, I just want to export nyan cat, maybe there is a simpler solution?
We don't care about efficiency.
Some self-documenting file format?

ASCII art?

```raw                    
     ]#########[    
    ]#XXX2XYSX##    
    ]ZoXXXr_/2XX _, 
    ]ZXXXX[o2]SX)o( 
 _, ]ZSXSu(nns__Sn( 
 "s_]ZXuX2)vq{oXq{n 
   "]ZuXXX)ns)oss)S 
    ]ZXX2X)oo{o{}oX 
    ]#oXXXs"o__s_r' 
   v,"!!!!!!   _    
   n' _     _  v    
```

No, I want colors!

Scrolling through the supported file type, I see HTML table.
That would be perfect!

![Gimp crashing](img/gimp_crash.png)

No luck for me?
It really looks like obscure rarely-used file formats are not super well supported?
Better call captain obvious for help?

By the way, I re-tried all those things a few times with Gimp versions around 2.10.38 and 3.0

Okay, why would an HTML table exporter fall over?
Transparency?
Animation frames?
Multiple layers?
All of the above?

Let's get rid of all those snares.
Let's copy the first frame into a blank new canvas with blue background.

![nyan cat in gimp with simplifications](img/gimp_simplified.png)

Then export as HTML table.
Starting from Gimp 3, it looks like selecting `HTML table` explicitly is required, since normal HTML export creates more fancy exports.
Too fancy for UCS-2.

![Gimp select file type HTML table](img/gimp_select_htmltable.png)

![Gimp warning about file extension not matching chosen file type](img/gimp_warn.png)

Whatever.
Probably the two types of Gimp HTML exports compete?
Whatever.

![Gimp Export Image as HTML Table](img/gimp_export_html.png)

Looks good.

A file [nyan.html](img/nyan.html) was created.

![nyan cat rendered in an HTML table in the browser](img/nyan_browser.png)

Awesome! :heart_eyes_cat:

This really looks like it was meant to be rendered by UEFI UCS-2 `BLOCKELEMENT_FULL_BLOCK`.

### Converting HTML Nyan Cat to Rust Vector

The [generated HTML file](img/nyan.html) really looks simple.

```bash
$ cat nyan.html | head -n20
<HTML>
<HEAD><TITLE>/home/user/git/uefi_nyan_80x25/drawing_nyan/img/nyan.html</TITLE></HEAD>
<BODY>
<H1>/home/user/git/uefi_nyan_80x25/drawing_nyan/img/nyan.html</H1>
<TABLE BORDER=2 CELLPADDING=4 CELLSPACING=0>
   <TR>
      <TD  BGCOLOR=#0200ff>
      &nbsp;
      </TD>
      <TD  BGCOLOR=#0200ff>
      &nbsp;
      </TD>
      <TD  BGCOLOR=#0200ff>
      &nbsp;
      </TD>
      <TD  BGCOLOR=#0200ff>
      &nbsp;
      </TD>
      <TD  BGCOLOR=#0200ff>
      &nbsp;
```

How many colors are in there?

```shell
$ grep '<TD \+BGCOLOR' nyan.html | sort | uniq
      <TD  BGCOLOR=#000000>
      <TD  BGCOLOR=#0200ff>
      <TD  BGCOLOR=#999999>
      <TD  BGCOLOR=#ff3399>
      <TD  BGCOLOR=#ff9999>
      <TD  BGCOLOR=#ff99ff>
      <TD  BGCOLOR=#ffcc99>
      <TD  BGCOLOR=#ffffff>
```

Only 8 colors?
That's perfect!

Let's parse that HTML file.
Probably using a regex is fine to parse HTML?
What does [stackoverflow say about this](https://stackoverflow.com/a/1732454)?
Yes, this link points to the epic answer that HTML is not a regular language, thus, a regular expression is the wrong tool.
And that is a great answer.
But did you notice the shell command above where we counted the colors?
`grep '<TD \+BGCOLOR' nyan.html`.
We _did_ just parse HTML with a regex!

Our HTML is well-formatted and trusted!
Often, we can parse many non-regular languages with a regex, _when the language was properly formatted beforehand_.
Look at the following example:

```html
<p>
   level 1
   <p>
      level 2
      <p>
         level 3
      </p>
   </p>
   <p>
      level 2 again
   </p>
   level 1 again
</p>
```

We can directly see the nesting level of each paragraph.  
If we want to extract everything at a specific nesting level, we just need to filter by the amount of leading whitespaces.
A simple exercise in a regex.
The number of whitespaces is basically a count to which nested block the current block belongs.
Essentially, the whitespaces correspond to the persisted stack memory of a pushdown automaton (the next powerful thing after a regex in the [Chomsky hierarchy](https://en.wikipedia.org/wiki/Chomsky_hierarchy)).

When working in larger code bases and doing a large-scale-change, often, well-formatting the target files and then going yolo and parsing with a regex can be very efficient.
Some people may get horrified by that approach, but, given we have the well-formatted part under control, it's super efficient and can be quite safe when done with proper care.

![normal distribution meme where left and right parse HTML with a regex while the majority claims that HTML is not regular. Created via https://imgflip.com/i/9bc045](img/not_regular_normal_meme.jpg)

Again, given we have the well-formatted part under control!
And we definitely have that here with the Gimp HTML export on my machine.

### Nyan HTML to Rust Array

Let's create a new Rust module `nyan.rs`.

```rust
use uefi::proto::console::text::Color::*;

pub const NYAN_40X25: [Color; 40*25] = [...]
```

Now we need to fill the `NYAN_40X25` with the Simple Text Output Protocol `uefi::proto::console::text::Color`s.

Do we need explicit memory management, type safety, the ability to scale a project and work in larger teams?
No?
Then let's quickly hack that in python.
I love python!
But don't let the project become large.

```python
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
```

And here it is, `nyan.rs`:

```rust
use uefi::proto::console::text::Color;
use uefi::proto::console::text::Color::*;

pub const NYAN_40X25: [Color; 40*25] = [Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Black, Black, Black, Black, Black, Black, Black, Black, Black, Black, Black, Black, Black, Black, Black, Black, Black, Black, Black, Black, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Black, Black, Brown, Brown, Brown, Brown, Brown, Brown, Brown, Brown, Brown, Brown, Brown, Brown, Brown, Brown, Brown, Brown, Brown, Brown, Brown, Brown, Black, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Black, Black, Brown, Brown, Brown, Brown, Brown, Brown, Brown, Brown, Brown, Brown, Brown, Brown, Brown, Brown, Brown, Brown, Brown, Brown, Brown, Brown, Black, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Black, Brown, Brown, Brown, Brown, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, Brown, Brown, Brown, Brown, Black, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Black, Brown, Brown, Brown, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, Magenta, LightMagenta, LightMagenta, Magenta, Magenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, Brown, Brown, Black, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Black, Brown, Brown, LightMagenta, LightMagenta, Magenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, Black, Black, Black, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, Brown, Black, Blue, Black, Black, Black, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Black, Brown, Brown, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, Black, LightGray, LightGray, LightGray, Black, LightMagenta, Magenta, LightMagenta, LightMagenta, LightMagenta, Brown, Black, Black, LightGray, LightGray, LightGray, Black, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Black, Brown, Brown, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, Black, LightGray, LightGray, LightGray, LightGray, Black, LightMagenta, LightMagenta, LightMagenta, LightMagenta, Brown, Black, LightGray, LightGray, LightGray, LightGray, Black, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Black, Brown, Brown, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, Black, LightGray, LightGray, LightGray, LightGray, Black, LightMagenta, LightMagenta, LightMagenta, LightMagenta, Brown, Black, LightGray, LightGray, LightGray, LightGray, Black, Blue, Blue, Blue, Blue, Black, Black, Black, Blue, Blue, Blue, Black, Brown, Brown, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, Magenta, LightMagenta, LightMagenta, Black, LightGray, LightGray, LightGray, LightGray, LightGray, Black, Black, Black, Black, Black, LightGray, LightGray, LightGray, LightGray, LightGray, Black, Blue, Blue, Blue, Black, LightGray, LightGray, LightGray, Black, Blue, Blue, Black, Brown, Brown, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, Black, LightGray, LightGray, LightGray, LightGray, LightGray, LightGray, LightGray, LightGray, LightGray, LightGray, LightGray, LightGray, LightGray, LightGray, LightGray, Black, Blue, Blue, Blue, Black, LightGray, LightGray, LightGray, Black, Black, Black, Black, Brown, Brown, LightMagenta, LightMagenta, LightMagenta, Magenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, Black, LightGray, LightGray, LightGray, LightGray, LightGray, LightGray, LightGray, LightGray, LightGray, LightGray, LightGray, LightGray, LightGray, LightGray, LightGray, LightGray, LightGray, Black, Blue, Blue, Blue, Black, Black, LightGray, LightGray, LightGray, LightGray, Black, Brown, Brown, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, Magenta, Black, LightGray, LightGray, LightGray, LightGray, White, Black, LightGray, LightGray, LightGray, LightGray, LightGray, LightGray, White, Black, LightGray, LightGray, LightGray, Black, Blue, Blue, Blue, Blue, Blue, Black, Black, LightGray, LightGray, Black, Brown, Brown, LightMagenta, Magenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, Black, LightGray, LightGray, LightGray, LightGray, Black, Black, LightGray, LightGray, LightGray, LightGray, Black, LightGray, Black, Black, LightGray, LightGray, LightGray, Black, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Black, Black, Black, Brown, Brown, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, Black, LightGray, LightRed, LightRed, LightRed, LightGray, LightGray, LightGray, LightGray, LightGray, LightGray, LightGray, LightGray, LightGray, LightGray, LightRed, LightRed, LightRed, Black, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Black, Black, Black, Brown, Brown, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, Black, LightGray, LightRed, LightRed, LightRed, LightGray, LightGray, LightGray, LightGray, LightGray, LightGray, LightGray, LightGray, LightGray, LightGray, LightRed, LightRed, LightRed, Black, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Black, Brown, Brown, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, Magenta, LightMagenta, LightMagenta, Black, LightGray, LightRed, LightRed, LightRed, LightGray, Black, LightGray, LightGray, LightGray, Black, LightGray, LightGray, Black, LightGray, LightRed, LightRed, LightRed, Black, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Black, Brown, Brown, Brown, LightMagenta, Magenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, Black, LightGray, LightGray, LightGray, LightGray, Black, Black, Black, Black, Black, Black, Black, Black, LightGray, LightGray, LightGray, Black, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Black, Black, Brown, Brown, Brown, Brown, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, LightMagenta, Black, Black, LightGray, LightGray, LightGray, LightGray, LightGray, LightGray, LightGray, LightGray, LightGray, LightGray, LightGray, Black, Black, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Black, LightGray, Black, Black, Black, Brown, Brown, Brown, Brown, Brown, Brown, Brown, Brown, Brown, Brown, Brown, Brown, Brown, Black, Black, Black, Black, Black, Black, Black, Black, Black, Black, Black, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Black, LightGray, LightGray, LightGray, Black, Black, Black, Black, Black, Black, Black, Black, Black, Black, Black, Black, Black, Black, Black, Black, Black, Black, Black, Black, Black, Black, LightGray, Black, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Black, LightGray, LightGray, LightGray, Black, Black, Black, Black, Black, Black, Black, Black, Black, Black, Black, Black, Black, Black, Black, Black, Black, Black, Black, Black, Black, Black, LightGray, Black, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Black, LightGray, LightGray, Black, Blue, Blue, Black, LightGray, LightGray, Black, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Black, Black, LightGray, LightGray, Black, Blue, Black, Black, LightGray, LightGray, Black, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Black, Black, Black, Blue, Blue, Blue, Black, Black, Black, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Black, Black, Black, Black, Blue, Blue, Blue, Blue, Black, Black, Black, Blue, Blue, Blue, Blue, Blue, Blue, Blue];
```

Technically, the file `nyan.rs` is still the original nyan cat?
So, technically, all rights reserved by prguitarman?
[IANAL](https://en.wikipedia.org/wiki/IANAL).

We are now ready to draw nyan cat in UCS-2 colors. :cat:

[back](../) | [next](../putting_together/)