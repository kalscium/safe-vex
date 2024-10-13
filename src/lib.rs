//! A modular, safe and data-orientated rust wrapper over the Purdue PROS library for vex

#![no_std]
#![feature(alloc_error_handler)]
#![feature(negative_impls)]
#![warn(missing_docs)]

mod bindings;

extern crate alloc;
