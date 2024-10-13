//! # Motor API

use crate::{bindings, error::PROSErr, port::SmartPort};

/// Sets the voltage linearly for a motor from `-127` to `127`
pub fn move_i8(port: SmartPort, reversed: bool, val: i8) -> Option<PROSErr> {
    PROSErr::parse(unsafe {
        bindings::motor_move((port as i16 * if reversed { -1 } else { 1 }) as u8, val as i32)
    })
}
