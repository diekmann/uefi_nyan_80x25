# Putting It All Together

## Drawing the `NYAN_40X25` Array.

Let'd draw the array!
Our simple program from the previous chapter is almost ready, [let's adapt it](https://github.com/diekmann/uefi_nyan_80x25/commit/e79bb0f93f2a5cb52ade54a5b478a8a76b95337f#diff-8604ed4b2e74eddb20dce286c605a4cfb983f26421d5efb8c730b277e5dc62c2).

```rust
#![no_main]
#![no_std]

use log::info;
use uefi::prelude::*;
use uefi::proto::console::text::Color::{Blue,Black};

mod nyan;

const BLOCKELEMENT_FULL_BLOCK: uefi::Char16 = unsafe {uefi::Char16::from_u16_unchecked(0x2588 as u16)};

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    let background = Blue;
    system::with_stdout(|stdout| -> uefi::Result {
        let must_mode_80x25 = stdout.modes().next().unwrap(); // the first one must be the 80x25 mode.
        stdout.set_mode(must_mode_80x25)?;

        // Paints the whole background blue. This is even a documented feature of `set_color`+`clear`.
        stdout.set_color(Black, background)?;
        stdout.clear()?;

        // Dump all modes.
        for m in stdout.modes() {
            info!("supported mode {}: {} {}", m.index(), m.columns(), m.rows());
        }
        
        for color in nyan::NYAN_40X25 {
            stdout.set_color(color, background)?;
            let mut s = uefi::CString16::new();
            s.push(BLOCKELEMENT_FULL_BLOCK);
            stdout.output_string(&s)?;
        }
        Ok(())
    }).expect("talking to EFI Simple Text Output Protocol went wrong");
    boot::stall(10_000_000);
    Status::SUCCESS
}
```

```bash
$ cargo run
```

![qemu showing two nyan cats](img/twonyan.png)

Wait?
Why are there two nyan cats?
Am I drunk?

Okay, we set the ~~resolution~~ mode to 80x25.
This means, we can draw 80 characters before an automated linebreak occurs.
Our nyan cat image was scaled down to 40x25.
If we draw it without explicit line breaks, because 80 = 40·2, we accidentally draw two nyan cats next to each other.
We also only drew 13 instead of 25 lines, that's why the debug text at the top still shows up.

Let's add a linebreak.

```diff
diff --git a/nyan/src/main.rs b/nyan/src/main.rs
index a2f09a4..ea76b1a 100644
--- a/nyan/src/main.rs
+++ b/nyan/src/main.rs
@@ -8,6 +8,7 @@ use uefi::proto::console::text::Color::{Blue,Black};
 mod nyan;
 
 const BLOCKELEMENT_FULL_BLOCK: uefi::Char16 = unsafe {uefi::Char16::from_u16_unchecked(0x2588 as u16)};
+const NEWLINE: uefi::Char16 = unsafe { uefi::Char16::from_u16_unchecked(b'\n' as u16) };
 
 #[entry]
 fn main() -> Status {
@@ -26,10 +27,13 @@ fn main() -> Status {
             info!("supported mode {}: {} {}", m.index(), m.columns(), m.rows());
         }
         
-        for color in nyan::NYAN_40X25 {
-            stdout.set_color(color, background)?;
+        for (i, color) in nyan::NYAN_40X25.iter().enumerate() {
+            stdout.set_color(*color, background)?;
             let mut s = uefi::CString16::new();
             s.push(BLOCKELEMENT_FULL_BLOCK);
+            if i%40 == 0 {
+                s.push(NEWLINE);
+            }
             stdout.output_string(&s)?;
         }
         Ok(())
```

![nyan cat in qemu, but something is wrong](img/nyan_strange.png)

Hmmm, :confused:

Oh right, UEFI is basically Windows-style.
We did UNIX-like `\n` linebreaks.
Let's try Windows-style `\r\n` with explicit carriage return.
It's the first time I actually see how a newline looks like if the cursor is not returned to the beginning of the line.

```diff
diff --git a/nyan/src/main.rs b/nyan/src/main.rs
index ea76b1a..c2e5a4b 100644
--- a/nyan/src/main.rs
+++ b/nyan/src/main.rs
@@ -8,6 +8,7 @@ use uefi::proto::console::text::Color::{Blue,Black};
 mod nyan;
 
 const BLOCKELEMENT_FULL_BLOCK: uefi::Char16 = unsafe {uefi::Char16::from_u16_unchecked(0x2588 as u16)};
+const CARRIAGE_RET: uefi::Char16 = unsafe { uefi::Char16::from_u16_unchecked(b'\r' as u16) };
 const NEWLINE: uefi::Char16 = unsafe { uefi::Char16::from_u16_unchecked(b'\n' as u16) };
 
 #[entry]
@@ -32,6 +33,7 @@ fn main() -> Status {
             let mut s = uefi::CString16::new();
             s.push(BLOCKELEMENT_FULL_BLOCK);
             if i%40 == 0 {
+                s.push(CARRIAGE_RET);
                 s.push(NEWLINE);
             }
             stdout.output_string(&s)?;
```

```bash
$ cargo run
```

![nyan cat rendered in qemu](img/nyan_rendered.png)

Nice!!

:tada: :smile_cat: :confetti_ball: :confetti_ball: :smile_cat: :tada:

**TODO** hardware

But let me take a note for later:
The proportions of those two nyan cats look quite good.
The `BLOCKELEMENT_FULL_BLOCK` is not a perfect square.
When downsizing nyan cat, we assumed we are drawing it as pixels.
But `BLOCKELEMENT_FULL_BLOCK` **TODO**
Start over.
Can do animation while at it.

[back](../)