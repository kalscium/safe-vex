use core::time::Duration;

use vex_rt::{robot, prelude::Peripherals, rtos::{Loop, Mutex}, select};
use crate::{context::Context, log::Log};

/// The time between ticks (runtime cycles)
pub const TICK_SPEED: u64 = 50;

/// A safe translation layer to convert the user defined Bot into a vex competition Robot struct.
pub struct Robot<T: for <'a> Bot<'a> + Sync + Send + 'static> {
    custom: Mutex<T>,
    context: Context,
}

pub trait Bot<'a> {
    /// Creates a new instance of a bot
    fn new(context: &'a Context) -> Self;
    /// Run each tick (runtime cycle) of `opcontrol`
    #[allow(unused_variables)]
    fn opcontrol(&'a mut self, context: &'a Context) {}
    /// Run each tick (runtime cycle) of `autonomous`
    #[allow(unused_variables)]
    fn autonomous(&'a mut self, context: &'a Context) {}
}

#[cfg(not(feature = "simulate"))]
macro_rules! vex_map {
    ($name:ident, $log:ident) => {
        #[inline]
        fn $name(&mut self, context: vex_rt::prelude::Context) {
            self.context.log($crate::log::Log::$log);
            let mut l = Loop::new(Duration::from_millis(TICK_SPEED));
            loop {
                self.context.log($crate::log::Log::Nothing);
                if let Some(mut custom) = self.custom.poll() {
                    custom.$name(&self.context);
                } else { self.context.log($crate::log::Log::RobotLockFailure) }

                select! {
                    _ = context.done() => break,
                    _ = l.select() => {
                        continue;
                    },
                }
            }
        }
    }
}

#[cfg(not(feature = "simulate"))]
impl<T: for <'a> Bot<'a> + Sync + Send + 'static> robot::Robot for Robot<T> {
    #[inline]
    fn new(peripherals: Peripherals) -> Self {
        let context = Context::new(peripherals);
        Self {
            custom: Mutex::new(T::new(&context)),
            context,
        }
    }

    vex_map!(opcontrol, OpControl);
    vex_map!(autonomous, Autonomous);

    #[inline]
    fn disabled<'a>(&mut self, _ctx: vex_rt::prelude::Context) {
        self.context.log(Log::Disabled);
    }

    #[inline]
    fn initialize(&mut self, _ctx: vex_rt::prelude::Context) {}
}

#[cfg(feature = "simulate")]
impl<T: for <'a> Bot<'a> + Sync + Send + 'static> robot::Robot for Robot<T> {
    #[inline]
    fn new(peripherals: Peripherals) -> Self {
        let context = Context::new(peripherals);
        Self {
            custom: Mutex::new(T::new(&context)),
            context,
        }
    }

    #[inline]
    fn opcontrol(&mut self, _ctx: vex_rt::prelude::Context) {
        self.context.log(Log::Disabled);
        self.context.log(Log::Autonomous);

        let mut l = Loop::new(Duration::from_millis(TICK_SPEED));
        let mut opcontrol = false;
        let mut tick = 0u16;
        loop {
            self.context.log(Log::Nothing);

            if !opcontrol && tick >= 15 * 1000 / TICK_SPEED as u16 {
                self.context.log(Log::Disabled);
                self.context.log(Log::OpControl);
                opcontrol = true;
            }

            if opcontrol {
                if let Some(mut custom) = self.custom.poll() {
                    custom.opcontrol(&self.context);
                } else { self.context.log(Log::RobotLockFailure) }
            } else {
                if let Some(mut custom) = self.custom.poll() {
                    custom.autonomous(&self.context);
                } else { self.context.log(Log::RobotLockFailure) }
            }

            select! {
                _ = l.select() => {
                    tick += 1;
                    continue;
                },
            }
        }
    }
}
