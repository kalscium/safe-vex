use vex_rt::{robot, prelude::Peripherals};

pub struct Robot<T: Bot + Sync + Send + 'static>(T);

pub trait Bot {
    fn new() -> Self;
}

impl<T: Bot + Sync + Send + 'static> robot::Robot for Robot<T> {
    #[inline]
    fn new(peripherals: Peripherals) -> Self {
        Self(T::new())
    }
}

struct Dave {
    
}

impl Bot for Dave {
    fn new() -> Self {
        Self {}
    }
}
