use vex_rt::{prelude::Peripherals, rtos::Mutex};
use crate::{log::{Logger, Log}, controller::Controller};

pub struct Context {
    perph: Peripherals,
    logger: Logger,
    pub logged_controller_disconnect: bool,
}

impl Context {
    pub fn new(perph: Peripherals) -> Self {
        Self {
            perph,
            logger: Logger::new(),
            logged_controller_disconnect: false,
        }
    }

    #[inline]
    pub fn log(&mut self, log: Log) {
        self.logger.push(log);
    }

    #[inline]
    pub fn flush_logs(&mut self) {
        todo!();
    }

    #[inline]
    pub fn controller<'a>(&'a self, this: &'a Mutex<Self>) -> Controller {
        Controller::new(&this, &self.perph.master_controller)
    }
}