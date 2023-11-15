#![no_main]
#![no_std]

use safe_vex::prelude::*;

struct Example;

impl Bot for Example {
    fn new(ctx: &Mutex<Context>) -> Self { Self }
}

entry!(Example);