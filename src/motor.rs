//! # Motor API

use crate::{bindings, error::{PROSErr, PROSResult}, port::SmartPort};

/// Gets the current voltage for a motor from `-12000` to `12000`
///
/// # Errors
///
/// - Returns `PROSErr::NoDev` if the port cannot be configured as a motor
pub fn get_voltage(port: SmartPort, reversed: bool) -> Result<i32, PROSErr> {
    unsafe {
        bindings::motor_get_voltage(port as i8 * if reversed { -1 } else { 1 })
    }.check()
}

/// Sets the voltage linearly for a motor from `-127` to `127`
///
/// # Errors
///
/// - Returns `PROSErr::NoDev` if the port cannot be configured as a motor
pub fn move_i8(port: SmartPort, reversed: bool, val: i8) -> Result<(), PROSErr> {
    unsafe {
        bindings::motor_move(port as i8 * if reversed { -1 } else { 1 }, val as i32)
    }.check().map(|_| ())
}

/// Sets the exact voltage for a motor from `-12000` to `12000`
///
/// # Errors
///
/// - Returns `PROSErr::NoDev` if the port cannot be configured as a motor
pub fn move_voltage(port: SmartPort, reversed: bool, val: i32) -> Result<(), PROSErr> {
    unsafe {
        bindings::motor_move_voltage(port as i8 * if reversed { -1 } else { 1 }, val)
    }.check().map(|_| ())
}
