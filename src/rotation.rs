//! # Rotation Sensor API

use crate::{bindings, error::{PROSErr, PROSResult}, port::SmartPort};

/// Resets the Rotation Sensor
///
/// Reset the current absolute position to be the same as the Rotation Sensor angle
///
/// # Errors
/// 
/// Returns `PROSErr::NoDev` if the port cannot be configured as a Rotation Sensor
pub fn reset(port: SmartPort) -> Result<(), PROSErr> {
    unsafe {
        bindings::rotation_reset(port as u8)
    }.check().map(|_| ())
}

/// Sets the Rotation Sensor's refresh interval in milliseconds
///
/// The rate may be specified in increments of 5ms, and will be rounded down to the nearest increment. The minimum allowable refresh rate is 5ms. The default rate is 10ms.
///
/// # Errors
/// 
/// Returns `PROSErr::NoDev` if the port cannot be configured as a Rotation Sensor
pub fn set_data_rate(port: SmartPort, rate: u32) -> Result<(), PROSErr> {
    unsafe {
        bindings::rotation_set_data_rate(port as u8, rate)
    }.check().map(|_| ())
}

/// Sets the Rotation Sensor position reading to a desired rotation value
///
/// # Errors
/// 
/// Returns `PROSErr::NoDev` if the port cannot be configured as a Rotation Sensor
pub fn set_position(port: SmartPort, position: u32) -> Result<(), PROSErr> {
    unsafe {
        bindings::rotation_set_position(port as u8, position)
    }.check().map(|_| ())
}

/// Resets the Rotation Sensor position to 0
///
/// # Errors
/// 
/// Returns `PROSErr::NoDev` if the port cannot be configured as a Rotation Sensor
pub fn reset_position(port: SmartPort) -> Result<(), PROSErr> {
    unsafe {
        bindings::rotation_reset_position(port as u8)
    }.check().map(|_| ())
}

/// Gets the Rotation Sensor's current position in centidegrees
///
/// # Errors
/// 
/// Returns `PROSErr::NoDev` if the port cannot be configured as a Rotation Sensor
pub fn get_position(port: SmartPort) -> Result<i32, PROSErr> {
    unsafe {
        bindings::rotation_get_position(port as u8)
    }.check()
}

/// Gets the Rotation Sensor's current velocity in centidegrees per second
///
/// # Errors
/// 
/// Returns `PROSErr::NoDev` if the port cannot be configured as a Rotation Sensor
pub fn get_velocity(port: SmartPort) -> Result<i32, PROSErr> {
    unsafe {
        bindings::rotation_get_velocity(port as u8)
    }.check()
}

/// Gets the Rotation Sensor's current angle in centidegrees (0-36000)
///
/// # Errors
/// 
/// Returns `PROSErr::NoDev` if the port cannot be configured as a Rotation Sensor
pub fn get_angle(port: SmartPort) -> Result<i32, PROSErr> {
    unsafe {
        bindings::rotation_get_angle(port as u8)
    }.check()
}

/// Sets the Rotation Sensor reversed flag
///
/// # Errors
/// 
/// Returns `PROSErr::NoDev` if the port cannot be configured as a Rotation Sensor
pub fn set_reversed(port: SmartPort, is_reversed: bool) -> Result<(), PROSErr> {
    unsafe {
        bindings::rotation_set_reversed(port as u8, is_reversed)
    }.check().map(|_| ())
}

/// Gets the Rotation Sensor's reversed flag
///
/// # Errors
/// 
/// Returns `PROSErr::NoDev` if the port cannot be configured as a Rotation Sensor
pub fn get_reversed(port: SmartPort) -> Result<bool, PROSErr> {
    unsafe {
        bindings::rotation_get_reversed(port as u8)
    }.check().map(|x| x != 0)
}
