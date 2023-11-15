use core::time::Duration;

use vex_rt::{robot, prelude::Peripherals, rtos::{Mutex, Loop}, select};
use crate::context::Context;

pub type TickType = u16;
pub const TICK_SPEED: u64 = 50;

pub struct Robot<T: Bot + Sync + Send + 'static> {
    custom: T,
    context: Mutex<Context>,
}

pub trait Bot {
    fn new(ctx: &Mutex<Context>) -> Self;
    #[allow(unused_variables)]
    fn opcontrol(&mut self, ctx: &Mutex<Context>, tick: TickType) {}
    #[allow(unused_variables)]
    fn autonomous(&mut self, ctx: &Mutex<Context>, tick: TickType) {}
    #[allow(unused_variables)]
    fn disabled(&mut self, ctx: &Mutex<Context>, tick: TickType) {}
}

macro_rules! vex_map {
    ($name:ident) => {
        #[inline]
        fn $name(&mut self, ctx: vex_rt::prelude::Context) {
            let mut l = Loop::new(Duration::from_millis(TICK_SPEED));
            let mut tick: TickType = 0;
            loop {
                self.custom.$name(&self.context, tick);

                select! {
                    _ = ctx.done() => break,
                    _ = l.select() => {
                        tick += 1;
                        continue;
                    },
                }
            }
        }
    }
}

impl<T: Bot + Sync + Send + 'static> robot::Robot for Robot<T> {
    #[inline]
    fn new(peripherals: Peripherals) -> Self {
        let context = Mutex::new(Context::new(peripherals));
        Self {
            custom: T::new(&context),
            context,
        }
    }

    vex_map!(opcontrol);
    vex_map!(autonomous);
    vex_map!(disabled);
}