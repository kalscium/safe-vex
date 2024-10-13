//! Raw Bindings to both the C Standard Library and also the Purdue PROS Library

#![allow(clippy::all)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(missing_docs)]

pub use libc::*;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
