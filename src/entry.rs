//! A safe wrapper for entry function definitions

/// A safe wrapper for entry function definitions
#[macro_export]
macro_rules! entry {
    // user-facing
    ($($entry:ident: $user:ident;)*) => {
        $(
            entry!(@internal $entry $user);
        )*
    };

    // internal

    // initialisation function
    (@internal initialize $user:ident) => {
        #[no_mangle]
        unsafe extern "C" fn initialize() {
            $user();
        }
    };

    // opcontrol function
    (@internal opcontrol $user:ident) => {
        #[no_mangle]
        unsafe extern "C" fn opcontrol() {
            $user();
        }
    };

    // autonomous function
    (@internal autonomous $user:ident) => {
        #[no_mangle]
        unsafe extern "C" fn autonomous() {
            $user();
        }
    };

    // disabled function
    (@internal disabled $user:ident) => {
        #[no_mangle]
        unsafe extern "C" fn disabled() {
            $user();
        }
    };
}
