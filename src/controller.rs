pub mod joystick;

use crate::{context::Context, log::Log};
use self::joystick::JoyStick;

pub struct Controller<'a>(&'a mut Context);

macro_rules! button {
    ($name:ident) => {
        /// Safely gets the activation of a button on the controller.
        /// Returns `false` if controller is disconnected
        #[inline]
        pub fn $name(&mut self) -> bool {
            match self.0.perph.master_controller.$name.is_pressed() {
                Ok(x) => {
                    if self.0.is_controller_disconnected {
                        self.0.log(Log::ControllerConnect);
                        self.0.is_controller_disconnected = false;
                    } x
                },
                Err(_) => {
                    if !self.0.is_controller_disconnected {
                        self.0.log(Log::ControllerDisconnect);
                        self.0.is_controller_disconnected = true;
                    }

                    false
                },
            }
        }
    }
}

macro_rules! joystick {
    ($name:ident) => {
        /// Safely gets the current state of a joystick.
        /// Returns the default state if the controller is disconnected
        #[inline]
        pub fn $name(&mut self) -> JoyStick {
            match (self.0.perph.master_controller.$name.get_x(), self.0.perph.master_controller.$name.get_y()) {
                (Ok(x), Ok(y)) => {
                    if self.0.is_controller_disconnected {
                        self.0.log(Log::ControllerConnect);
                        self.0.is_controller_disconnected = false;
                    } JoyStick { x, y }
                },
                _ => {
                    if !self.0.is_controller_disconnected {
                        self.0.log(Log::ControllerDisconnect);
                        self.0.is_controller_disconnected = true;
                    } JoyStick { x: 0, y: 0 }
                },
            }
        }
    }
}

impl<'a> Controller<'a> {
    /// Gets the current state of the controller
    #[inline]
    pub(crate) fn new(context: &'a mut Context) -> Self {
        Self(context)
    }

    button!(a);
    button!(b);
    button!(x);
    button!(y);
    button!(up);
    button!(down);
    button!(left);
    button!(right);
    button!(l1);
    button!(l2);
    button!(r1);
    button!(r2);

    joystick!(left_stick);
    joystick!(right_stick);
}