pub mod joystick;

use vex_rt::{controller::Controller as VexControl, rtos::Mutex};
use crate::{context::Context, log::Log};
use self::joystick::JoyStick;

pub struct Controller<'a> {
    context: &'a Mutex<Context>,
    controller: &'a VexControl,
}

macro_rules! button {
    ($name:ident) => {
        #[inline]
        pub fn $name(&self) -> bool {
            match self.controller.$name.is_pressed() {
                Ok(x) => {
                    let context = &mut self.context.lock();
                    if context.logged_controller_disconnect {
                        context.log(Log::ControllerConnect);
                        context.logged_controller_disconnect = false;
                    } x
                },
                Err(_) => {
                    let context = &mut self.context.lock();
                    if !context.logged_controller_disconnect {
                        context.log(Log::ControllerDisconnect);
                        context.logged_controller_disconnect = true;
                    }

                    false
                },
            }
        }
    }
}

macro_rules! joystick {
    ($name:ident) => {
        #[inline]
        pub fn $name(&self) -> JoyStick {
            match (self.controller.$name.get_x(), self.controller.$name.get_y()) {
                (Ok(x), Ok(y)) => {
                    let mut context = self.context.lock();
                    if context.logged_controller_disconnect {
                        context.log(Log::ControllerConnect);
                        context.logged_controller_disconnect = false;
                    } JoyStick { x, y }
                },
                _ => {
                    let mut context = self.context.lock();
                    if !context.logged_controller_disconnect {
                        context.log(Log::ControllerDisconnect);
                        context.logged_controller_disconnect = true;
                    } JoyStick { x: 0, y: 0 }
                },
            }
        }
    }
}

impl<'a> Controller<'a> {
    #[inline]
    pub fn new(context: &'a Mutex<Context>, controller: &'a VexControl) -> Self {
        Self {
            controller,
            context,
        }
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