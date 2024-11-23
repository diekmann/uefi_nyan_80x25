#![no_main]
#![no_std]

use log::info;
use uefi::prelude::*;
use uefi::proto::console::text::Color::{Blue,Black};

mod nyan;

const BLOCKELEMENT_FULL_BLOCK: uefi::Char16 = unsafe {uefi::Char16::from_u16_unchecked(0x2588 as u16)};
const CARRIAGE_RET: uefi::Char16 = unsafe { uefi::Char16::from_u16_unchecked(b'\r' as u16) };
const NEWLINE: uefi::Char16 = unsafe { uefi::Char16::from_u16_unchecked(b'\n' as u16) };

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
        
        for (i, color) in nyan::NYAN_40X25.iter().enumerate() {
            stdout.set_color(*color, background)?;
            let mut s = uefi::CString16::new();
            s.push(BLOCKELEMENT_FULL_BLOCK);
            if i%40 == 0 {
                s.push(CARRIAGE_RET);
                s.push(NEWLINE);
            }
            stdout.output_string(&s)?;
        }
        Ok(())
    }).expect("talking to EFI Simple Text Output Protocol went wrong");
    boot::stall(10_000_000);
    Status::SUCCESS
}
