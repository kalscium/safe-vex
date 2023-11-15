use vex_rt::{controller::Controller as VexControl, rtos::Mutex};

use crate::{context::Context, log::Log};

pub struct Controller<'a> {
    context: &'a Mutex<Context>,
    controller: &'a VexControl,
    logged_disconnect: bool
}

macro_rules! button {
    ($name:ident) => {
        #[inline]
        pub fn $name(&mut self) -> bool {
            match self.controller.$name.is_pressed() {
                Ok(x) => {
                    if self.logged_disconnect {
                        self.context.lock().log(Log::ControllerConnect);
                    } self.logged_disconnect = false;
                    x
                },
                Err(_) => {
                    if !self.logged_disconnect {
                        self.context.lock().log(Log::ControllerDisconnect);
                        self.logged_disconnect = true;
                    }

                    false
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
            logged_disconnect: false,
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