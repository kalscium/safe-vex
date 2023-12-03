use core::time::Duration;

use vex_rt::{robot, prelude::Peripherals, rtos::{Loop, Mutex}, select};
use crate::{context::Context, log::Log};

/// The time between ticks (runtime cycles)
pub const TICK_SPEED: u64 = 50;

/// A safe translation layer to convert the user defined Bot into a vex competition Robot struct.
pub struct Robot<T: for <'a> Bot<'a, TESTING> + Sync + Send + 'static, const TESTING: bool> {
    custom: Mutex<T>,
    context: Mutex<Context>,
}

pub trait Bot<'a, const TESTING: bool> {
    /// Creates a new instance of a bot
    fn new(context: &'a Context) -> Self;
    /// Run each tick (runtime cycle) of `opcontrol`
    #[allow(unused_variables)]
    fn opcontrol(&'a self, context: &'a Mutex<Context>) {}
    /// Run each tick (runtime cycle) of `autonomous`
    #[allow(unused_variables)]
    fn autonomous(&'a self, context: &'a Mutex<Context>) {}
}

macro_rules! vex_map {
    ($name:ident, $log:ident) => {
        #[inline]
        fn $name(&mut self, context: vex_rt::prelude::Context) {
            self.context.lock().log($crate::log::Log::$log);
            let mut l = Loop::new(Duration::from_millis(TICK_SPEED));
            loop {
                self.context.lock().log($crate::log::Log::Nothing);
                self.custom.lock().$name(&self.context);

                select! {
                    _ = context.done() => break,
                    _ = l.select() => {
                        self.context.lock().tick += 1;
                        continue;
                    },
                }
            }
        }
    }
}

impl<T: for <'a> Bot<'a, false> + Sync + Send + 'static> robot::Robot for Robot<T, false> {
    #[inline]
    fn new(peripherals: Peripherals) -> Self {
        let context = Context::new(peripherals);
        Self {
            custom: Mutex::new(T::new(&context)),
            context: Mutex::new(context),
        }
    }

    vex_map!(opcontrol, OpControl);
    vex_map!(autonomous, Autonomous);

    #[inline]
    fn disabled(&mut self, _ctx: vex_rt::prelude::Context) {
        self.context.lock().log(Log::Disabled);
    }

    #[inline]
    fn initialize(&mut self, _ctx: vex_rt::prelude::Context) {}
}

impl<T: for <'a> Bot<'a, true> + Sync + Send + 'static> robot::Robot for Robot<T, true> {
    #[inline]
    fn new(peripherals: Peripherals) -> Self {
        let context = Context::new(peripherals);
        Self {
            custom: Mutex::new(T::new(&context)),
            context: Mutex::new(context),
        }
    }

    #[inline]
    fn opcontrol(&mut self, _ctx: vex_rt::prelude::Context) {
        self.context.lock().log(Log::Disabled);
        self.context.lock().log(Log::Autonomous);

        let mut l = Loop::new(Duration::from_millis(TICK_SPEED));
        let mut opcontrol = false;
        loop {
            self.context.lock().log(Log::Nothing);

            if !opcontrol && self.context.lock().tick >= 15 * 1000 / TICK_SPEED as u16 {
                self.context.lock().log(Log::Disabled);
                self.context.lock().log(Log::OpControl);
                opcontrol = true;
            }

            if opcontrol {
                self.custom.lock().opcontrol(&self.context);
            } else {
                self.custom.lock().autonomous(&self.context);
            }

            select! {
                _ = l.select() => {
                    self.context.lock().tick += 1;
                    continue;
                },
            }
        }
    }
}