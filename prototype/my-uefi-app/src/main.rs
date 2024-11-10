#![no_main]
#![no_std]

use core::fmt::Write;

use log::info;
use uefi::prelude::*;
use uefi::print;
use uefi::proto::console::text;

mod nyan;

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    info!("Hello world!");
    //let console_handle = boot::locate_handle_buffer(uefi::boot::SearchType::ByProtocol(&uefi::proto::console::text));
    //let st = uefi::table::system_table_raw().expect("global system table pointer is not set");
    system::with_stdout(|stdout| -> uefi::Result {
        set_mode(stdout)?;
        let m = stdout.current_mode().unwrap().unwrap();
        draw(stdout, m.rows(), m.columns())?;
        Ok(())
    }).expect("main stdout failed");
    info!("Exiting, ...");
    boot::stall(10_000_000);
    Status::SUCCESS
}


fn set_mode(stdout: &mut text::Output) -> uefi::Result {
    let mode = stdout.current_mode()?;
    match mode {
        None => info!("initially: no mode"),
        Some(m) => info!("initially: mode {m:?}"),
    }
    for m in stdout.modes() {
        info!("supported mode {}: {} {}", m.index(), m.rows(), m.columns());
    }
    let must_mode_80x25 = stdout.modes().next().unwrap(); // the first one must be the 80x25 mode.
    boot::stall(500_000);
    stdout.set_mode(must_mode_80x25)
}

fn draw(stdout: &mut text::Output, _rows: usize, _cols: usize) -> uefi::Result {
    const BLOCKELEMENT_FULL_BLOCK: uefi::Char16 = unsafe {uefi::Char16::from_u16_unchecked(0x2588 as u16)};
    const CARRIAGE_RET: uefi::Char16 = unsafe { uefi::Char16::from_u16_unchecked(b'\r' as u16) };
    const NEWLINE: uefi::Char16 = unsafe { uefi::Char16::from_u16_unchecked(b'\n' as u16) };

    // draw everything with background blue.
    stdout.set_color(text::Color::Black, text::Color::Blue)?;
    stdout.clear()?;

    let mut cnt = 0;
    for color in nyan::NYAN_40x25 {
        cnt += 1;
        let mut s = uefi::CString16::new();
        stdout.set_color(color, text::Color::Blue)?;
        s.push(BLOCKELEMENT_FULL_BLOCK);
        if cnt % 40 == 0 {
            s.push(CARRIAGE_RET);
            s.push(NEWLINE);
        }
        stdout.output_string(&s)?;
    }
    boot::stall(5_000_000);
    stdout.set_color(text::Color::Black, text::Color::Blue)?;
    Ok(())
}


fn colortest(stdout: &mut text::Output) -> uefi::Result {
    // https://uefi.org/specs/UEFI/2.9_A/12_Protocols_Console_Support.html#simple-text-output-protocol
    // UNICODE DRAWING CHARACTERS
    //const NULL: uefi::Char16 = unsafe {uefi::Char16::from_u16_unchecked(0x0 as u16)};
    //const BOXDRAW_HORIZONTAL: uefi::Char16 = unsafe {uefi::Char16::from_u16_unchecked(0x2500 as u16)};
    //const BOXDRAW_DOUBLE_HORIZONTAL: uefi::Char16 = unsafe {uefi::Char16::from_u16_unchecked(0x2550 as u16)};
    //const BOXDRAW_VERTICAL_RIGHT_DOUBLE: uefi::Char16 = unsafe {uefi::Char16::from_u16_unchecked(0x255e as u16)};
    const BLOCKELEMENT_FULL_BLOCK: uefi::Char16 = unsafe {uefi::Char16::from_u16_unchecked(0x2588 as u16)};
    //const BLOCKELEMENT_LIGHT_SHADE: uefi::Char16 = unsafe {uefi::Char16::from_u16_unchecked(0x2591 as u16)};

    // draw everything with background blue.
    stdout.set_color(text::Color::Black, text::Color::Blue)?;
    stdout.clear()?;

    for color in [text::Color::Black, text::Color::Blue, text::Color::Green, text::Color::Cyan, text::Color::Red, text::Color::Magenta, text::Color::Brown, text::Color::LightGray, text::Color::DarkGray, text::Color::LightBlue, text::Color::LightGreen, text::Color::LightCyan, text::Color::LightRed, text::Color::LightMagenta, text::Color::Yellow, text::Color::White] {
        let mut s = uefi::CString16::new();
        stdout.set_color(color, text::Color::Blue)?;
        s.push(BLOCKELEMENT_FULL_BLOCK);
        stdout.output_string(&s)?;
    }
    boot::stall(5_000_000);
    stdout.set_color(text::Color::Black, text::Color::Blue)?;
    Ok(())
}