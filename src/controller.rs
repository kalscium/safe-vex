//! # Controller API

use alloc::ffi::CString;

use crate::{bindings, error::{PROSErr, PROSResult}};

/// A controller that you can read from
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Controller {
    /// The master controller
    Master = 0,
    /// The partner controller
    Partner = 1,
}

/// An Analog Joystick on the Controller
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum ControllerAnalog {
    /// The x axis of the left joystick
    LeftX = 0,
    /// The y axis of the left joystick
    LeftY = 1,
    /// The x axis of the right joystick
    RightX = 2,
    /// The y axis of the right joystick
    RightY = 3,
}

/// A digital (button) on the controller
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum ControllerDigital {
    /// The first trigger on the left side of the controller
    L1 = 6,
    /// The second trigger on the left side of the controller
    L2,
    /// The first trigger on the right side of the controller
    R1,
    /// The second trigger on the right side of the controller
    R2,
    /// The up arrow on the left arrow pad of the controller
    Up,
    /// The down arrow on the left arrow pad of the controller
    Down,
    /// The left arrow on the left arrow pad of the controller
    Left,
    /// The right arrow on the left arrow pad of the controller
    Right,
    /// The ‘X’ button on the right button pad of the controller
    X,
    /// The ‘B’ button on the right button pad of the controller
    B,
    /// The ‘Y’ button on the right button pad of the controller
    Y,
    /// The ‘A’ button on the right button pad of the controller
    A,
}

/// Returns the analog value of a controller
///
/// # Errors
///
/// Returns `PROSErr::Access` if another resource is currently trying to access the controller
/// Returns `0` if the controller is not connected
pub fn get_analog(controller: Controller, analog: ControllerAnalog) -> Result<i8, PROSErr> {
    unsafe {
        bindings::controller_get_analog(controller as u32, analog as u32)
    }.check().map(|x| x as i8)
}

/// Returns the digital value of a controller button
///
/// # Errors
///
/// Returns `PROSErr::Access` if another resource is currently trying to access the controller
/// Returns `false` if the controller is not connected
pub fn get_digital(controller: Controller, digital: ControllerDigital) -> Result<bool, PROSErr> {
    unsafe {
        bindings::controller_get_digital(controller as u32, digital as u32)
    }.check().map(|x| x != 0)
}

/// Rumbles the controller
///
/// The rumble pattern is a string consisting of the characters '.', '-', and ' ', where dots are short rumbles, dashes are long rumbles, and spaces are pauses. Maximum supported length is 8 characters
///
/// # Errors
///
/// Returns `PROSErr::Access` if another resource is currently trying to access the controller
pub fn rumble(controller: Controller, rumble_pattern: &str) -> Result<(), PROSErr> {
    // rumble the controller
    unsafe {
        let rumble_pattern = CString::new(rumble_pattern).unwrap();
        bindings::controller_rumble(controller as u32, rumble_pattern.as_ptr() as *const u8)
    }.check().map(|_| ())
}
