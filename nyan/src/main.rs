#![no_main]
#![no_std]

use log::info;
use uefi::prelude::*;
use uefi::proto::console::text::Color::{Black, Blue};

mod nyan;

const BLOCKELEMENT_FULL_BLOCK: uefi::Char16 = unsafe { uefi::Char16::from_u16_unchecked(0x2588 as u16) };
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

        for _ in 0..100 {
            for frame in [nyan::NYAN_80X25_01, nyan::NYAN_80X25_02, nyan::NYAN_80X25_03, nyan::NYAN_80X25_04, nyan::NYAN_80X25_05, nyan::NYAN_80X25_06, nyan::NYAN_80X25_07, nyan::NYAN_80X25_08, nyan::NYAN_80X25_09, nyan::NYAN_80X25_10, nyan::NYAN_80X25_11, nyan::NYAN_80X25_12] {
                stdout.clear()?;
                let mut s = uefi::CString16::new();
                let mut prev_color = frame[0];
                stdout.set_color(prev_color, background)?;
                for color in frame[0 .. frame.len()-1].iter() {
                    if (prev_color as usize) != (*color as usize) {
                        stdout.set_color(prev_color, background)?;
                        stdout.output_string(&s)?;
                        s = uefi::CString16::new();
                    }
                    s.push(BLOCKELEMENT_FULL_BLOCK);
                    prev_color = color.clone();
                }
                boot::stall(70_000);
            }
        }
        Ok(())
    })
    .expect("talking to EFI Simple Text Output Protocol went wrong");
    boot::stall(10_000_000);
    Status::SUCCESS
}
