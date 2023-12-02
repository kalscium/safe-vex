use core::time::Duration;

use vex_rt::{robot, prelude::Peripherals, rtos::Loop, select};
use crate::context::Context;

/// The time between ticks (runtime cycles)
pub const TICK_SPEED: u64 = 50;

/// A safe translation layer to convert the user defined Bot into a vex competition Robot struct.
pub struct Robot<T: for <'a> Bot<'a> + Sync + Send + 'static> {
    custom: T,
    context: Context,
}

pub trait Bot<'a> {
    /// Creates a new instance of a bot
    fn new(context: &'a Context) -> Self;
    /// Run each tick (runtime cycle) of `opcontrol`
    #[allow(unused_variables)]
    fn opcontrol(&'a mut self, context: &'a mut Context) {}
    /// Run each tick (runtime cycle) of `autonomous`
    #[allow(unused_variables)]
    fn autonomous(&'a mut self, context: &'a mut Context) {}
    /// Run each tick (runtime cycle) of `autonomous`
    #[allow(unused_variables)]
    fn disabled(&'a mut self, context: &'a mut Context) {}
}

macro_rules! vex_map {
    ($name:ident, $log:ident) => {
        #[inline]
        fn $name(&mut self, context: vex_rt::prelude::Context) {
            self.context.log($crate::log::Log::$log);
            let mut l = Loop::new(Duration::from_millis(TICK_SPEED));
            loop {
                self.context.log($crate::log::Log::Nothing);
                self.custom.$name(&mut self.context);

                select! {
                    _ = context.done() => break,
                    _ = l.select() => {
                        self.context.tick += 1;
                        continue;
                    },
                }
            }
        }
    }
}

impl<T: for <'a> Bot<'a> + Sync + Send + 'static> robot::Robot for Robot<T> {
    #[inline]
    fn new(peripherals: Peripherals) -> Self {
        let context = Context::new(peripherals);
        Self {
            custom: T::new(&context),
            context,
        }
    }

    vex_map!(opcontrol, OpControl);
    vex_map!(autonomous, Autonomous);
    vex_map!(disabled, Disabled);
}