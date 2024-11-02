//! Raw Bindings to both the C Standard Library and also the Purdue PROS Library

#![allow(clippy::all)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(missing_docs)]

pub use libc::*;

// need to manually declare until https://github.com/rust-lang/libc/issues/1995 is resolved.
extern "C" {
    /// Gets a mutable pointer to the C/C++ `errno` value
    pub fn __errno() -> *mut i32;
}

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
