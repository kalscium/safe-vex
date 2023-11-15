use vex_rt::{controller::Controller as VexControl, rtos::Mutex};

use crate::{context::Context, log::Log};

pub struct Controller<'a> {
    context: &'a Mutex<Context>,
    controller: &'a VexControl,
}

macro_rules! button {
    ($name:ident) => {
        #[inline]
        pub fn $name(&mut self) -> bool {
            match self.controller.$name.is_pressed() {
                Ok(x) => {
                    let context = &mut self.context.lock();
                    if context.logged_controller_disconnect {
                        context.log(Log::ControllerConnect);
                    } context.logged_controller_disconnect = false;
                    x
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

impl<'a> Controller<'a> {
    #[inline]
    pub const fn new(context: &'a Mutex<Context>, controller: &'a VexControl) -> Self {
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
}