//! # Error API

/// Possible PROS Errors
#[derive(Debug, Clone, Copy)]
#[repr(i32)]
pub enum PROSErr {
    /// No such device
    NoDev = 19,
}

impl PROSErr {
    /// Parses and returns a `PROSErr` from an `i32` if the error is valid
    pub fn parse(err: i32) -> Option<Self> {
        Some(match err {
            19 => Self::NoDev,

            // if none match
            _ => return None,
        })
    }
}
