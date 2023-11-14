use vex_rt::{robot, prelude::Peripherals};
use crate::context::Context;

pub struct Robot<T: Bot + Sync + Send + 'static> {
    custom: T,
    context: Context,
}

pub trait Bot {
    fn new(context: &Context) -> Self;
}

impl<T: Bot + Sync + Send + 'static> robot::Robot for Robot<T> {
    #[inline]
    fn new(peripherals: Peripherals) -> Self {
        let context = Context::new(peripherals);
        Self {
            custom: T::new(&context),
            context,
        }
    }
}