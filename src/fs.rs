//! Utilities for safely accessing the file system of v5 brain

use core::ffi::CStr;
use alloc::{boxed::Box, ffi::CString, string::{String, ToString}};
use byte_strings::c_str;
use crate::{bindings, error::{self, PROSErr, PROSResult}};

/// Returns if there is an SD card plugged into the brain or not
pub fn is_available() -> bool {
    unsafe {
        bindings::usd_is_installed() != 0
    }
}

/// Lists the files **(NOT DIRECTORIES)** in a directory in the SD card
///
/// # Errors
///
/// - Returns `PROSErr::IO` on low-level hardware I/O error
/// - Returns `PROSErr::Invalid` when the path is a file, the length is invalid, or the path is invalid
/// - Returns `PROSErr::NoEntry` when the path cannot be found
/// - Returns `PROSErr::Access` when access is denied
/// - Returns `PROSErr::Exists`  when access is denied
/// - Returns `PROSErr::ReadOnlyFS` when the SD card is write protected
/// - Returns `PROSErr::NXIO` when the drive number is invalid or not an FAT32 drive
/// - Returns `PROSErr::NoBuffSpace` when the drive has no work area
/// - Returns `PROSErr::NoMoreFiles` when there are too many open files
///
/// # Warning
/// use a path of `\` to list the files in the main directory **NOT** `/usd/`` DO **NOT** PREPEND YOUR PATHS WITH `/usd/`
pub fn list_files(path: &str) -> Result<Box<[String]>, PROSErr> {
    /// The size of the file list buffer
    const BUFF_SIZE: i32 = 1024;

    // initialise the buffer
    let mut buffer = [0u8; BUFF_SIZE as usize];

    // list and write to buffer
    unsafe {
        let path = CString::new(path).unwrap();
        bindings::usd_list_files(
            path.as_ptr() as *const u8,
            (&mut buffer) as *mut u8,
            BUFF_SIZE,
        )
    }.check()?;

    // creates a cstring from the buffer and splits it into separate entries
    let cstring = CStr::from_bytes_until_nul(&buffer).unwrap();

    // split and return it
    let split = cstring.to_str()
        .unwrap()
        .split("\n")
        .map(|x| x.to_string())
        .collect::<Box<[String]>>();
    Ok(split)
}

/// A safe wrapper over a C filestream with write permissions
pub struct FileWrite {
    /// internal pointer to the C file
    pointer: *mut bindings::FILE,
}

// /// A safe wrapper over a C filestream with read permissions
// pub struct FileRead {
//     /// internal pointer to the C file
//     pointer: *mut bindings::FILE,
// }

impl FileWrite {
    /// Creates a new file
    ///
    /// # Errors
    ///
    /// - Returns `PROSErr::IO` on low-level hardware I/O error
    /// - Returns `PROSErr::Invalid` when the length or path is invalid
    /// - Returns `PROSErr::NoEntry` when the path cannot be found
    /// - Returns `PROSErr::Access` when access is denied
    /// - Returns `PROSErr::Exists`  when access is denied
    /// - Returns `PROSErr::ReadOnlyFS` when the SD card is write protected
    /// - Returns `PROSErr::NXIO` when the drive number is invalid or not an FAT32 drive
    /// - Returns `PROSErr::NoBuffSpace` when the drive has no work area
    /// - Returns `PROSErr::NoMoreFiles` when there are too many open files
    ///
    /// # Warning
    /// **All** paths **must** start with `/usd/`, if you don't it won't work
    pub fn create(path: &str) -> Result<FileWrite, PROSErr> {
        // cast the strings to c strings
        let path = CString::new(path).unwrap();

        // obtain the file pointer
        let pointer = unsafe {
            bindings::fopen(
                path.as_ptr() as *const u8,
                c_str!("w").as_ptr() as *const u8,
            )
        };

        // check for errors
        if pointer.is_null() {
            return Err(error::from_errno());
        }

        // return the valid pointer
        Ok(FileWrite {
            pointer
        })
    }

    /// Opens a file that **already** exists
    ///
    /// # Errors
    ///
    /// - Returns `PROSErr::IO` on low-level hardware I/O error
    /// - Returns `PROSErr::Invalid` when the length or path is invalid
    /// - Returns `PROSErr::NoEntry` when the path cannot be found
    /// - Returns `PROSErr::Access` when access is denied
    /// - Returns `PROSErr::Exists`  when access is denied
    /// - Returns `PROSErr::ReadOnlyFS` when the SD card is write protected
    /// - Returns `PROSErr::NXIO` when the drive number is invalid or not an FAT32 drive
    /// - Returns `PROSErr::NoBuffSpace` when the drive has no work area
    /// - Returns `PROSErr::NoMoreFiles` when there are too many open files
    ///
    /// # Warning
    /// **All** paths **must** start with `/usd/`, if you don't it won't work
    pub fn open(path: &str) -> Result<FileWrite, PROSErr> {
        // cast the strings to c strings
        let path = CString::new(path).unwrap();

        // obtain the file pointer
        let pointer = unsafe {
            bindings::fopen(
                path.as_ptr() as *const u8,
                c_str!("a").as_ptr() as *const u8,
            )
        };

        // check for errors
        if pointer.is_null() {
            return Err(error::from_errno());
        }

        // return the valid pointer
        Ok(FileWrite {
            pointer
        })
    }

    /// Writes a string to the file
    ///
    /// # WARNING
    ///
    /// # Errors
    ///
    /// - Returns `PROSErr::IO` on low-level hardware I/O error
    /// - Returns `PROSErr::Access` when access is denied
    /// - Returns `PROSErr::Exists`  when access is denied
    /// - Returns `PROSErr::ReadOnlyFS` when the SD card is write protected
    /// - Returns `PROSErr::NXIO` when the drive number is invalid or not an FAT32 drive
    /// - Returns `PROSErr::NoBuffSpace` when the drive has no work area
    /// - Returns `PROSErr::NoMoreFiles` when there are too many open files
    pub fn write(&mut self, string: &str) -> Result<(), PROSErr> {
        // cast to c string
        let c_string = CString::new(string).unwrap();

        // write to the file
        let error = unsafe {
            bindings::fputs(c_string.as_ptr() as *const u8, self.pointer)
        };

        // check for error
        if error == -1 { // eof
            return Err(error::from_errno());
        }

        Ok(())
    }

    /// Closes the filestream safely
    pub fn close(self) {
        unsafe {
            bindings::fclose(self.pointer);
        }
    }
}

impl Drop for FileWrite {
    fn drop(&mut self) {
        unsafe {
            bindings::fclose(self.pointer);
        }
    }
}
