use vex_rt::prelude::Peripherals;

pub struct Context {
    perph: Peripherals,
}

impl Context {
    pub fn new(perph: Peripherals) -> Self {
        Self {
            perph,
        }
    }
}