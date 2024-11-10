//! # Inertial Sensor API

use crate::{bindings, error::{PROSErr, PROSResult}, port::SmartPort};

/// Calibrate the IMU Sensor
///
/// Calibration takes approximately `2` seconds, but this function only blocks until the IMU status flag is set properly to E_IMU_STATUS_CALIBRATING, with a minimum blocking time of 5ms
///
/// # Errors
///
/// - Returns `PROSErr::NoDev` if the port cannot be configured as a Inertial Sensor
/// - Returns `PROSErr::Again` if the sensor is already currently calibrating, or time out setting the status flag
pub fn reset(port: SmartPort) -> Result<(), PROSErr> {
    unsafe {
        bindings::imu_reset(port as u8)
    }.check().map(|_| ())
}

/// Gets the Inertial Sensor's heading relative to the initial direction of it's x-axis
///
/// This value is bounded by `0..=360`///
///
/// # Errors
///
/// - Returns `PROSErr::NoDev` if the port cannot be configured as a Inertial Sensor
/// - Returns `PROSErr::Again` if the sensor is still calibrating
pub fn get_heading(port: SmartPort) -> Result<f64, PROSErr> {
    unsafe {
        bindings::imu_get_heading(port as u8)
    }.check()
}

/// Gets the Inertial Sensor's heading relative to the initial direction of it's x-axis
///
/// This value is bounded by `-180..=180`. Clockwise rotations are represented with positive degree values, while counterclockwise rotations are represented with negative ones
///
/// # Errors
///
/// - Returns `PROSErr::NoDev` if the port cannot be configured as a Inertial Sensor
/// - Returns `PROSErr::Again` if the sensor is still calibrating
pub fn get_yaw(port: SmartPort) -> Result<f64, PROSErr> {
    unsafe {
        bindings::imu_get_yaw(port as u8)
    }.check()
}

/// Gets the Inertial Sensor's heading relative to the initial direction of it's x-axis
///
/// This value is bounded by `-180..=180`. Clockwise rotations are represented with positive degree values, while counterclockwise rotations are represented with negative ones
///
/// # Errors
///
/// - Returns `PROSErr::NoDev` if the port cannot be configured as a Inertial Sensor
/// - Returns `PROSErr::Again` if the sensor is still calibrating
pub fn get_pitch(port: SmartPort) -> Result<f64, PROSErr> {
    unsafe {
        bindings::imu_get_pitch(port as u8)
    }.check()
}

/// Gets the Inertial Sensor's heading relative to the initial direction of it's x-axis
///
/// This value is bounded by `-180..=180`. Clockwise rotations are represented with positive degree values, while counterclockwise rotations are represented with negative ones
///
/// # Errors
///
/// - Returns `PROSErr::NoDev` if the port cannot be configured as a Inertial Sensor
/// - Returns `PROSErr::Again` if the sensor is still calibrating
pub fn get_roll(port: SmartPort) -> Result<f64, PROSErr> {
    unsafe {
        bindings::imu_get_roll(port as u8)
    }.check()
}

/// Resets all 5 values of the Interial Sensor to 0
///
/// # Errors
///
/// - Returns `PROSErr::NoDev` if the port cannot be configured as a Inertial Sensor
/// - Returns `PROSErr::Again` if the sensor is still calibrating
pub fn tare(port: SmartPort) -> Result<(), PROSErr> {
    unsafe {
        bindings::imu_tare(port as u8)
    }.check().map(|_| ())
}

/// Sets the Inertial Sensor's refresh interval in milliseconds
///
/// # Errors
///
/// - Returns `PROSErr::NoDev` if the port cannot be configured as a Inertial Sensor
/// - Returns `PROSErr::Again` if the sensor is still calibrating
pub fn set_data_rate(port: SmartPort, rate: u32) -> Result<(), PROSErr> {
    unsafe {
        bindings::imu_set_data_rate(port as u8, rate)
    }.check().map(|_| ())
}

/// Raw Accelerometer values
pub struct Acceleration {
    /// accelleration in the x axis
    pub x: f64,
    /// accelleration in the y axis
    pub y: f64,
    /// accelleration in the z axis
    pub z: f64,
}

/// Gets the Inertial Sensor's raw accelerometer values
///
/// # Errors
///
/// - Returns `PROSErr::NoDev` if the port cannot be configured as a Inertial Sensor
/// - Returns `PROSErr::Again` if the sensor is still calibrating
pub fn get_accel(port: SmartPort) -> Result<Acceleration, PROSErr> {
    unsafe {
        let accel = bindings::imu_get_accel(port as u8);
        accel.x.check()?; // check for errors

        Ok(Acceleration {
            x: accel.x,
            y: accel.y,
            z: accel.z,
        })
    }
}
