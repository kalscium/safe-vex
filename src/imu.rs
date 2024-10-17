//! # Inertial Sensor API

use crate::{bindings, error::PROSErr, port::SmartPort};

/// Calibrate the IMU Sensor
///
/// Calibration takes approximately `2` seconds, but this function only blocks until the IMU status flag is set properly to E_IMU_STATUS_CALIBRATING, with a minimum blocking time of 5ms
///
/// # Errors
///
/// - Returns `PROSErr::NoDev` if the port cannot be configured as a Inertial Sensor
/// - Returns `PROSErr::Again` if the sensor is already currently calibrating, or time out setting the status flag
pub fn reset(port: SmartPort) -> Result<(), PROSErr> {
    PROSErr::parse(unsafe {
        bindings::imu_reset(port as u8)
    })
}

/// Gets the Inertial Sensor's heading relative to the initial direction of it's x-axis
///
/// This value is bounded by `0..=360`. Clockwise rotations are represented with positive degree values, while counterclockwise rotations are represented with negative ones
///
/// # Errors
///
/// - Returns `PROSErr::NoDev` if the port cannot be configured as a Inertial Sensor
/// - Returns `PROSErr::Again` if the sensor is still calibrating
pub fn get_heading(port: SmartPort) -> Result<f64, PROSErr> {
    // get the heading of the sensor
    let heading = unsafe {
        bindings::imu_get_heading(port as u8)
    };

    // check for errors
    PROSErr::parse(heading as i32)?;

    // return heading
    Ok(heading)
}

/// Resets all 5 values of the Interial Sensor to 0
///
/// # Errors
///
/// - Returns `PROSErr::NoDev` if the port cannot be configured as a Inertial Sensor
/// - Returns `PROSErr::Again` if the sensor is still calibrating
pub fn tare(port: SmartPort) -> Result<(), PROSErr> {
    // perform the tare and wrap any errors
    PROSErr::parse(unsafe {
        bindings::imu_tare(port as u8)
    })
}
