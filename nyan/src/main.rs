#![no_main]
#![no_std]

use log::info;
use uefi::prelude::*;

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    info!("Hello world!");
    let s: &uefi::CStr16 = cstr16!("Hello, World! From raw EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL");
    system::with_stdout(|stdout| -> uefi::Result{
        stdout.output_string(&s)?;
        Ok(())
    }).expect("talking to EFI Simple Text Output Protocol went wrong");
    boot::stall(10_000_000);
    Status::SUCCESS
}