//! # ADI API

use crate::{bindings, error::PROSErr, port::AdiPort};

/// An Adi configuration
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum AdiConfig {
    /// An analog Adi voltage input
    AnalogIn = 0,
    /// An analog Adi voltage output
    AnalogOut = 1,
    /// A digital Adi voltage input
    DigitalIn = 2,
    /// A digital Adi voltage output
    DigitalOut = 3,
}

/// Sets the configurations for a specified ADI port
pub fn set_config(port: AdiPort, config: AdiConfig)  {
    // set the config for the adi port
    unsafe {
        bindings::adi_port_set_config(port as u8, config as u32);
    }
}

/// **Warning:** ADI port must be configured prior to this function call
/// 
/// Sends an outbound digital signal to a **configured** ADI port
///
/// # Errors
/// 
/// - Returns `PROSErr::AddrInUse` if the ADI port is not configured correctly
///
/// # Safety
///
/// This function is marked unsafe due to it depending on the Adi port having previously being configured as the kind of Adi port that is expected by this function
pub unsafe fn digital_write(port: AdiPort, val: bool) -> Option<PROSErr> {
    // write the digital value to the adi port
    let code = unsafe {
        bindings::adi_digital_write(port as u8, val)
    };

    // return the parsed return code
    PROSErr::parse(code)
}

/// **Warning:** ADI port must be configured prior to this function call
/// 
/// Reads an inbound digital signal from a **configured** ADI port
///
/// # Errors
/// 
/// - Returns `PROSErr::AddrInUse` if the ADI port is not configured correctly
///
/// # Safety
///
/// This function is marked unsafe due to it depending on the Adi port having previously being configured as the kind of Adi port that is expected by this function
pub unsafe fn digital_read(port: AdiPort) -> Result<bool, PROSErr> {
    // read the digital value from the adi port
    let code = unsafe {
        bindings::adi_digital_read(port as u8)
    };

    // check for errors
    if let Some(err) = PROSErr::parse(code) {
        return Err(err);
    }

    // return the digital output
    Ok(code != 0)
}
