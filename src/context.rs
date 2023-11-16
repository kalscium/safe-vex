use vex_rt::prelude::Peripherals;
use crate::{log::{Logger, Log}, controller::Controller};

pub type TickType = u16;

/// The context (current state) of the robot's execution
pub struct Context {
    pub(crate) perph: Peripherals,
    logger: Logger,
    /// if it has been logged yet that the controller has disconnected
    pub logged_controller_disconnect: bool,
    /// The current tick (cycle) within the robot's execution
    pub tick: TickType,
}

impl Context {
    /// Creates a new context for the robot
    pub fn new(perph: Peripherals) -> Self {
        Self {
            perph,
            logger: Logger::new(),
            logged_controller_disconnect: false,
            tick: 0,
        }
    }

    /// Logs a log to the context's log pile
    #[inline]
    pub fn log(&mut self, log: Log) {
        self.logger.push(log);
    }

    /// Wipes the logs in the lop pile and also prints them to the console screen
    #[inline]
    pub fn flush_logs(&mut self) {
        todo!();
    }

    /// Gets the current state of the controller safely
    #[inline]
    pub fn controller<'a>(&'a mut self) -> Controller {
        Controller::new(self)
    }
}