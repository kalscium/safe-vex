//! # Motor API

use crate::{bindings, error::PROSErr, port::SmartPort};

/// Gets the current voltage for a motor from `-12000` to `12000`
///
/// # Errors
///
/// - Returns `PROSErr::NoDev` if the port cannot be configured as a motor
pub fn get_voltage(port: SmartPort, reversed: bool) -> Result<i32, PROSErr> {
    // get the voltage of the motor
    let code = unsafe {
        bindings::motor_get_voltage(port as i8 * if reversed { -1 } else { 1 })
    };

    // check for errors
    if let Some(err) = PROSErr::parse(code) {
        return Err(err);
    }
    
    // return voltage
    Ok(code)
}

/// Sets the voltage linearly for a motor from `-127` to `127`
///
/// # Errors
///
/// - Returns `PROSErr::NoDev` if the port cannot be configured as a motor
pub fn move_i8(port: SmartPort, reversed: bool, val: i8) -> Option<PROSErr> {
    PROSErr::parse(unsafe {
        bindings::motor_move(port as i8 * if reversed { -1 } else { 1 }, val as i32)
    })
}

/// Sets the exact voltage for a motor from `-12000` to `12000`
///
/// # Errors
///
/// - Returns `PROSErr::NoDev` if the port cannot be configured as a motor
pub fn move_voltage(port: SmartPort, reversed: bool, val: i8) -> Option<PROSErr> {
    PROSErr::parse(unsafe {
        bindings::motor_move_voltage(port as i8 * if reversed { -1 } else { 1 }, val as i32)
    })
}
