#![no_main]
#![no_std]

use safe_vex::{bot::Bot, entry, context::Context};

struct Example;

impl Bot for Example {
    fn new(ctx: &Context) -> Self { Self }
}

entry!(Example);