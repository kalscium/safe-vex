//! A safe wrapper for entry function definitions

/// A safe wrapper for entry function definitions
#[macro_export]
macro_rules! entry {
    // user-facing
    ($($entry:ident: $user:tt;)*) => {
        $(
            $crate::entry!(@internal $entry $user);
        )*
    };

    // internal

    // initialisation function
    (@internal initialize $user:tt) => {
        #[inline]
        #[no_mangle]
        unsafe extern "C" fn initialize() {
            $user();
        }
    };

    // opcontrol function
    (@internal opcontrol $user:tt) => {
        #[inline]
        #[no_mangle]
        unsafe extern "C" fn opcontrol() {
            $user();
        }
    };

    // autonomous function
    (@internal autonomous $user:tt) => {
        #[inline]
        #[no_mangle]
        unsafe extern "C" fn autonomous() {
            $user();
        }
    };

    // disabled function
    (@internal disabled $user:tt) => {
        #[inline]
        #[no_mangle]
        unsafe extern "C" fn disabled() {
            $user();
        }
    };

    // if nothing matches
    (@internal $invalid:ident $user:tt) => {
        compile_error!(concat!("entry macro error: entrypoint `", stringify!($invalid), "` does not exist"));
    };
}
