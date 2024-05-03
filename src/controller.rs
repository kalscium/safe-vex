//! Contains anything to do with the controller

use vex_rt::peripherals::Peripherals;
use self::joystick::JoyStick;
pub mod joystick;

/// Represents the state of a controller
#[derive(Debug, Clone)]
pub struct Controller {
    pub disconnected: bool,
    
    // buttons
    
    /// the `a` button on the controller
    pub a: bool,
    /// the `b` button on the controller
    pub b: bool,
    /// the `x` button on the controller
    pub x: bool,
    /// the `y` button on the controller
    pub y: bool,

    /// the `up` button on the controller
    pub up: bool,
    /// the `down` button on the controller
    pub down: bool,
    /// the `left` button on the controller
    pub left: bool,
    /// the `right` button on the controller
    pub right: bool,

    /// the `l1` button on the controller
    pub l1: bool,
    /// the `l2` button on the controller
    pub l2: bool,
    /// the `r1` button on the controller
    pub r1: bool,
    /// the `r2` button on the controller
    pub r2: bool,

    // joysticks

    /// the left joystick of the controller
    pub left_stick: JoyStick,
    /// the right joystick of the controller
    pub right_stick: JoyStick,
}

macro_rules! button {
    ($button:ident $perph:ident $disconnected:ident) => {
        let $button = match $perph.master_controller.$button.is_pressed() {
           Ok(x) => x,
           Err(_) => {
               $disconnected = true;
               false
           }
        };
    }
}

macro_rules! joystick {
    ($name:ident $perph:ident $disconnected:ident) => {
        let $name = match ($perph.master_controller.$name.get_x(), $perph.master_controller.$name.get_y()) {
            (Ok(x), Ok(y)) => JoyStick { x, y },
            _ => {
                $disconnected = true;
                JoyStick { x: 0, y: 0 }
            }
        };
    }
}
impl Controller {
    #[inline]
    pub(crate) fn new(perph: &Peripherals) -> Self {
        let mut disconnected = false;

        button!(a perph disconnected);
        button!(b perph disconnected);
        button!(x perph disconnected);
        button!(y perph disconnected);
        button!(up perph disconnected);
        button!(down perph disconnected);
        button!(left perph disconnected);
        button!(right perph disconnected);
        button!(l1 perph disconnected);
        button!(l2 perph disconnected);
        button!(r1 perph disconnected);
        button!(r2 perph disconnected);

        joystick!(left_stick perph disconnected);
        joystick!(right_stick perph disconnected);
        
        Self {
            disconnected,
            a,
            b,
            x,
            y,
            up,
            down,
            left,
            right,
            l1,
            l2,
            r1,
            r2,
            left_stick,
            right_stick,
        }
    }
}
