#![no_main]
#![no_std]

use core::fmt::Write;

use uefi::prelude::*;
use uefi::proto::console::text::Color::*;

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    let colors = [
        Black,
        Blue,
        Green,
        Cyan,
        Red,
        Magenta,
        Brown,
        LightGray,
        DarkGray,
        LightBlue,
        LightGreen,
        LightCyan,
        LightRed,
        LightMagenta,
        Yellow,
        White,
    ];
    system::with_stdout(|stdout| -> uefi::Result {
        for background in colors {
            for color in colors {
                stdout.set_color(color, background)?;
                stdout.output_string(cstr16!("XXX"))?;
            }
            stdout.write_char('\n').unwrap(); // core::fmt::Error, not uefi::Error
        }
        Ok(())
    }).expect("talking to EFI Simple Text Output Protocol went wrong");
    boot::stall(10_000_000);
    Status::SUCCESS
}
