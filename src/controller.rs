//! # Controller API

use crate::{bindings, error::PROSErr};

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

/// Returns the analog value of a controller, if the controller is connected
///
/// # Errors
///
/// Returns `PROSErr::Access` if another resource is currently trying to access the controller
pub fn get_analog(controller: Controller, analog: ControllerAnalog) -> Result<Option<i8>, PROSErr> {
    // get the analog value of the controller joystick
    let code = unsafe {
        bindings::controller_get_analog(controller as u32, analog as u32)
    };

    // check for errors
    if let Some(err) = PROSErr::parse(code) {
        return Err(err);
    }

    // make sure the controller is connected
    if code == 0 {
        return Ok(None);
    }

    // return valid analog controller input
    Ok(Some(code as i8))
}

/// Returns the digital value of a controller button, if the controller is connected
///
/// # Errors
///
/// Returns `PROSErr::Access` if another resource is currently trying to access the controller
pub fn get_digital(controller: Controller, digital: ControllerDigital) -> Result<Option<bool>, PROSErr> {
    // get the analog value of the controller joystick
    let code = unsafe {
        bindings::controller_get_digital(controller as u32, digital as u32)
    };

    // check for errors
    if let Some(err) = PROSErr::parse(code) {
        return Err(err);
    }

    // make sure the controller is connected
    if code == 0 {
        return Ok(None);
    }

    // return valid analog controller input
    Ok(Some(code == 1))
}
