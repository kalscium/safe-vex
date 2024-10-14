//! # Motor API

use crate::{bindings, error::PROSErr, port::SmartPort};

/// Gets the current voltage for a motor from `-12000` to `12000`
///
/// # Errors
///
/// - Returns `PROSErr::NXIO` if the motor port is invalid (*shouldn't* be possible)
/// - Returns `PROSErr::NoDev` if the port cannot be configured as a motor
pub fn get_voltage(port: SmartPort, reversed: bool) -> Result<i32, PROSErr> {
    let code = unsafe { // get the return code
        bindings::motor_get_voltage(port as i8 * if reversed { -1 } else { 1 })
    };
    let err = PROSErr::parse(code); // parse for an error *(if there is one)*
    err.map_or(Ok(code), Err) // return error if there is one, otherwise the return code
}

/// Sets the voltage linearly for a motor from `-127` to `127`
///
/// # Errors
///
/// - Returns `PROSErr::NXIO` if the motor port is invalid (*shouldn't* be possible)
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
/// - Returns `PROSErr::NXIO` if the motor port is invalid (*shouldn't* be possible)
/// - Returns `PROSErr::NoDev` if the port cannot be configured as a motor
pub fn move_voltage(port: SmartPort, reversed: bool, val: i8) -> Option<PROSErr> {
    PROSErr::parse(unsafe {
        bindings::motor_move_voltage(port as i8 * if reversed { -1 } else { 1 }, val as i32)
    })
}
