use vex_rt::prelude::Peripherals;
use crate::log::{Logger, Log};

pub struct Context {
    perph: Peripherals,
    logger: Logger,
}

impl Context {
    pub fn new(perph: Peripherals) -> Self {
        Self {
            perph,
            logger: Logger::new(),
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
}