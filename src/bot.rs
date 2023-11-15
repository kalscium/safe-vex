use vex_rt::{robot, prelude::Peripherals, rtos::Mutex};
use crate::{context::Context, controller::Controller};

pub struct Robot<T: Bot + Sync + Send + 'static> {
    custom: T,
    context: Mutex<Context>,
}

pub trait Bot {
    fn new(context: &Mutex<Context>) -> Self;
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
}