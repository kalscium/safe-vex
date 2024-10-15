//! A safe wrapper for entry function definitions

/// A safe wrapper for entry function definitions
#[macro_export]
macro_rules! entry {
    // user-facing
    ($($entry:ident => $body:stmt;)*) => {
        $(
            $crate::entry!(@internal $entry $body);
        )*
    };

    // internal

    // initialisation function
    (@internal initialize $body:stmt) => {
        #[inline]
        #[no_mangle]
        unsafe extern "C" fn initialize() {
            $body
        }
    };

    // opcontrol function
    (@internal opcontrol $body:stmt) => {
        #[inline]
        #[no_mangle]
        unsafe extern "C" fn opcontrol() {
            $body
        }
    };

    // autonomous function
    (@internal autonomous $body:stmt) => {
        #[inline]
        #[no_mangle]
        unsafe extern "C" fn autonomous() {
            $body
        }
    };

    // disabled function
    (@internal disabled $body:stmt) => {
        #[inline]
        #[no_mangle]
        unsafe extern "C" fn disabled() {
            $body
        }
    };

    // if nothing matches
    (@internal $invalid:ident $body:stmt) => {
        compile_error!(concat!("entry macro error: entrypoint `", stringify!($invalid), "` does not exist"));
    };
}
