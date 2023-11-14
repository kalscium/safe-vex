#![no_main]
#![no_std]

use safe_vex::{bot::Bot, entry};

struct Example;

impl Bot for Example {
    fn new() -> Self { Self }
}

entry!(Example);