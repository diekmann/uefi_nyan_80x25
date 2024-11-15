#![no_main]
#![no_std]

use log::info;
use uefi::prelude::*;


const BOXDRAW_HORIZONTAL: uefi::Char16 = unsafe {uefi::Char16::from_u16_unchecked(0x2500 as u16)};
const BOXDRAW_VERTICAL_HORIZONTAL_DOUBLE: uefi::Char16 = unsafe {uefi::Char16::from_u16_unchecked(0x256a as u16)};
const GEOMETRICSHAPE_UP_TRIANGLE: uefi::Char16 = unsafe {uefi::Char16::from_u16_unchecked(0x25b2 as u16)};

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    info!("Hello world!");
    system::with_stdout(|stdout| -> uefi::Result{
        let mut s = uefi::CString16::new();
        s.push(BOXDRAW_HORIZONTAL);
        s.push(BOXDRAW_HORIZONTAL);
        s.push(BOXDRAW_HORIZONTAL);
        s.push(BOXDRAW_VERTICAL_HORIZONTAL_DOUBLE);
        s.push(BOXDRAW_VERTICAL_HORIZONTAL_DOUBLE);
        s.push(BOXDRAW_VERTICAL_HORIZONTAL_DOUBLE);
        s.push(GEOMETRICSHAPE_UP_TRIANGLE);
        s.push(GEOMETRICSHAPE_UP_TRIANGLE);
        s.push(GEOMETRICSHAPE_UP_TRIANGLE);
        stdout.output_string(&s)?;
        Ok(())
    }).expect("talking to EFI Simple Text Output Protocol went wrong");
    boot::stall(10_000_000);
    Status::SUCCESS
}