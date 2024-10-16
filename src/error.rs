//! # Error API

/// Possible PROS Errors
#[derive(Debug, Clone, Copy)]
#[repr(i32)]
pub enum PROSErr {
    /// Permission denied
    Access = 13,
    /// No such device
    NoDev = 19,
    /// Address already in use or not configured correctly
    AddrInUse = 112,
}

impl PROSErr {
    /// Parses and returns a `PROSErr` from an `i32` if the error is valid
    pub fn parse(err: i32) -> Result<(), Self> {
        Err(match err {
            13 => Self::Access,
            19 => Self::NoDev,
            112 => Self::AddrInUse,

            // if none match
            _ => return Ok(()),
        })
    }
}
