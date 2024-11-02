//! # Error API

use crate::bindings;

// need to manually declare until https://github.com/rust-lang/libc/issues/1995 is resolved.
extern "C" {
    /// Gets a mutable pointer to the C/C++ `errno` value
    pub fn __errno() -> *mut i32;
}

/// Possible PROS Errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PROSErr {
    /// No errors
    None = 0,
    /// No more processes
    Again = 11,
    /// Permission denied
    Access = 13,
    /// No such device
    NoDev = 19,
    /// Address already in use or not configured correctly
    AddrInUse = 112,
}

/// Generates a [`PROSError`] from the value of `errno` for the current task
pub fn from_errno() -> PROSErr {
    match unsafe { *__errno() } {
        0 => PROSErr::None,
        11 => PROSErr::Again,
        13 => PROSErr::Access,
        19 => PROSErr::NoDev,
        112 => PROSErr::AddrInUse,
        _ => unreachable!(),
    }
}

/// A type which may contain a [`PROSErr`] depending upon a sentinel value that represents errors
///
/// Implementations are provided for `i32`, `f64` and `*mut T` values based on either PRO's sentinel error values (`PROS_ERR` or `PROS_ERR_F` in C/C++) or a NULL pointer
pub trait PROSResult: Sized {
    /// Checks if the type is a valid (success value), giving an appropriate
    /// error otherwise.
    fn check(self) -> Result<Self, PROSErr>;
}

impl PROSResult for i32 {
    fn check(self) -> Result<Self, PROSErr> {
        if self == bindings::PROS_ERR_ {
            Err(from_errno())
        } else {
            Ok(self)
        }
    }
}

impl PROSResult for f64 {
    fn check(self) -> Result<Self, PROSErr> {
        if self == bindings::PROS_ERR_F_ {
            Err(from_errno())
        } else {
            Ok(self)
        }
    }
}

impl<T> PROSResult for *mut T {
    fn check(self) -> Result<Self, PROSErr> {
        if self.is_null() {
            Err(from_errno())
        } else {
            Ok(self)
        }
    }
}
