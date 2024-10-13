//! # Motor API

use crate::{bindings, port::SmartPort};

/// Sets the voltage linearly for a motor from `-127` to `127`
pub fn move_i8(port: SmartPort, reversed: bool, val: i8) -> i32 {
    unsafe {
        bindings::motor_move((port as i16 * if reversed { -1 } else { 1 }) as u8, val as i32)
    }
}
