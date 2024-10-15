//! A modular, safe and data-orientated rust wrapper over the Purdue PROS library for vex

#![no_std]
#![feature(alloc_error_handler)]
#![feature(negative_impls)]
#![warn(missing_docs)]

pub mod bindings;

extern crate alloc;

pub mod allocator;
pub mod entry;
pub mod io;
pub mod error;
pub mod port;
pub mod motor;
pub mod adi;
pub mod controller;

/// Handles the program's panics
#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    crate::io::eprintln!(
        "panic occurred!: {:#?}",
        info,
    );

    unsafe {
        libc::exit(1)
    }
}
