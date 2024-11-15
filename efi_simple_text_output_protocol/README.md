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


## Outputting some Strings

Currently, our `Hello, World` is

```rust
#![no_main]
#![no_std]

use log::info;
use uefi::prelude::*;

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    info!("Hello world!");
    boot::stall(10_000_000);
    Status::SUCCESS
}
```

But this `log::info` abstracts away the raw `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL` we want to talk to.
Let's figure out how the `info!` macro actually writes to the screen.
It somehow must use the `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL` with a handle it got from the efi system table.

Looking into the definition of the `info` macro is not helpful, since this macro lives in the `log` crate, and that crate is not specific for UEFI.

So let's look at what `uefi::helpers::init()` does.
This looks interesting: It calls `logger::init`, which lives in the `uefi` crate.
I think we are on the right track.
Here is the definition of `init`, from `uefi-0.33.0/src/helpers/logger.rs`:

```rust
/// Set up logging
///
/// This is unsafe because you must arrange for the logger to be reset with
/// disable() on exit from UEFI boot services.
pub unsafe fn init() {
    // Connect the logger to stdout.
    system::with_stdout(|stdout| {
        LOGGER.set_output(stdout);
    });

    // Set the logger.
    log::set_logger(&LOGGER).unwrap(); // Can only fail if already initialized.

    // Set logger max level to level specified by log features
    log::set_max_level(log::STATIC_MAX_LEVEL);
}
```

So this is how this generic `log` crate gets initialized to talk to the EFI Simple Text Output Protocol.
And `system::with_stdout` is exactly what we are looking for.
It's documentation is

```rust
/// Call `f` with the [`Output`] protocol attached to stdout.
```

and the implementation does exactly what we thought it should do:
Get the output protocol pointer fro the efi system table and wrap it in a safe way into the `stdout` variable:

```rust
/// Call `f` with the [`Output`] protocol attached to stdout.
///
/// # Panics
///
/// This function will panic if called after exiting boot services, or if stdout
/// is not available.
pub fn with_stdout<F, R>(f: F) -> R
where
    F: Fn(&mut Output) -> R,
{
    let st = table::system_table_raw_panicking();
    // SAFETY: valid per requirements of `set_system_table`.
    let st = unsafe { st.as_ref() };
    // The I/O protocols cannot be used after exiting boot services.
    assert!(!st.boot_services.is_null(), "boot services are not active");
    assert!(!st.stdout.is_null(), "stdout is not available");

    let stdout: *mut Output = st.stdout.cast();

    // SAFETY: `Output` is a `repr(transparent)` wrapper around the raw output
    // type. The underlying pointer in the system table is assumed to be valid.
    let stdout = unsafe { &mut *stdout };

    f(stdout)
}
```

Great!
So, `system::with_stdout` is the function we want to use!

Let's see if we can use it.

```rust
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
```

The [`cstr16` macro](https://docs.rs/uefi/latest/uefi/prelude/macro.cstr16.html) is really convenient, since it handles the UCS-2 strings for us.
Without it, we would probably have to write down our string manually as an array of 16 bit encoded characters.

```bash
$ cargo run
```

![Hello, World! From raw EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL](img/hello_simple.png)

Nice!

need allocator for uefi::CString16::new()

Step 5
Intro UEFI simple output proto.
Few colors. BU so has Nyan. Perfect match.

HELLO WORLD, in color

Find the interesting characters in the docs.

Color test
On real hardware
Photo 
Like blue background.


Step 6
Standardize on graphics mode. Qemu comes up with?
Thinkpad comes up with?



[back](../)