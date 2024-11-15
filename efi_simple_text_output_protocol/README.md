# The EFI Simple Text Output Protocol

The [EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL](https://uefi.org/specs/UEFI/2.10/12_Protocols_Console_Support.html#efi-simple-text-output-protocol) provides a way to output text.
It's a feature provided by the ~~BIOS~~ UEFI.

An UEFI app can use it to output strings to the screen.

In UEFI land, the string encoding is [UCS-2](https://en.wikipedia.org/wiki/Universal_Coded_Character_Set), which combines the downsides of UTF-16 with the non-support of UCS-2 in any modern programming language.

To ask UEFI to do something, each UEFI app is basically passed a pointer to the [`EFI_SYSTEM_TABLE`](https://uefi.org/specs/UEFI/2.10/04_EFI_System_Table.html?highlight=efi_system_tabl#efi-system-table) in the `main` function.
This `EFI_SYSTEM_TABLE` is full of pointers to all the services provided by UEFI.
One of these services is the `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL`.

About 5 years ago, I [experimented with a minimal and raw UEFI application](https://github.com/diekmann/uefi_playground/tree/master) —a C and Rust hybrid— to just print "Hello, World" from scratch.
Fortunately, the world has moved on and we no longer need such raw from scratch setups.
We will trust in the Rust [`uefi` crate](https://docs.rs/uefi/latest/uefi/) to provide a lot of convenience for us.
That's the crate which also provides the great [Rust UEFI book](https://rust-osdev.github.io/uefi-rs/).

[back](../)