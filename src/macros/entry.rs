#[macro_export]
/// Specifies the entrypoint for the robot.
///
/// # Examples
///
/// ```
/// #![no_std]
/// #![no_main]
///
/// use safe_vex::prelude::*;
///
/// struct FooBot;
///
/// impl Robot for FooBot {
///     fn new() -> Self {
///         FooBot
///     }
/// }
///
/// entry!(FooBot);
/// ```
macro_rules! entry {
    ($entry:ty) => {
        static ROBOT: $crate::vex_rt::once::Once<$crate::vex_rt::robot::Competition<$crate::bot::Robot<$entry>>> =
            $crate::vex_rt::once::Once::new();

        #[no_mangle]
        unsafe extern "C" fn initialize() {
            ROBOT.call_once(|| {
                $crate::vex_rt::robot::Competition::new($crate::vex_rt::robot::Robot::new(unsafe {
                    $crate::vex_rt::peripherals::Peripherals::new()
                }))
            });
        }

        #[no_mangle]
        extern "C" fn opcontrol() {
            ROBOT.wait().opcontrol();
        }

        #[no_mangle]
        extern "C" fn autonomous() {
            ROBOT.wait().autonomous();
        }

        #[no_mangle]
        extern "C" fn disabled() {
            ROBOT.wait().disabled();
        }
    };
}

#[macro_export]
macro_rules! bind {
    () => {
        compile_error!("binds must have at least one condition");
    };

    ($head:expr, $($tail:expr),* => $func:expr) => {
        $crate::bind!($head, $tail => () $func)
    };
    
    ($head:expr, $($tail:expr),* => ($($extra:expr),*) $func:expr) => {
        $head.bind(&|x| $crate::bind!($($tail),* => ($($extra,)* x) $func))
    };

    ($tail:expr => ($extra:expr) $func:expr) => {
        $tail.bind(&|x| $func(($($extra,)* x)))
    };
}