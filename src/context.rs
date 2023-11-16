use alloc::string::ToString;
use vex_rt::{prelude::Peripherals, io::println};
use crate::{log::{Logger, Log}, controller::Controller, colour_format};

pub type TickType = u16;

/// The context (current state) of the robot's execution
pub struct Context {
    pub(crate) perph: Peripherals,
    logger: Logger,
    /// Detects if the controller is disconnected in the current runtime cycle
    pub is_controller_disconnected: bool,
    /// The current tick (cycle) within the robot's execution
    pub tick: TickType,
}

impl Context {
    /// Creates a new context for the robot
    pub fn new(perph: Peripherals) -> Self {
        Self {
            perph,
            logger: Logger::new(),
            is_controller_disconnected: false,
            tick: 0,
        }
    }

    /// Logs a log to the context's log pile
    #[inline]
    pub fn log(&mut self, log: Log) {
        self.logger.push(log);
    }

    /// Wipes the logs in the log pile and also prints them to the console screen
    #[inline]
    pub fn flush_logs(&mut self) {
        self.logger.flush(|log, i| {
            let tick = colour_format![blue("( "), yellow(&i.to_string()), blue(" ) ")];
            println!("{}", match log {
                Log::Other(x) => colour_format![raw(&tick), cyan("Custom Log"), blue(": "), none(x)],
                Log::Autonomous => colour_format![raw(&tick), cyan("Autonomous period "), green("started")],
                Log::OpControl => colour_format![raw(&tick), cyan("Driver Control period "), green("started")],
                Log::Disabled => colour_format![raw(&tick), cyan("Disabled period "), green("started")],
                Log::ControllerDisconnect => colour_format![raw(&tick), cyan("Controller "), red("disconnect")],
                Log::ControllerConnect => colour_format![raw(&tick), cyan("Controller "), green("connected")],
                Log::MotorDisconnect(port) => colour_format![raw(&tick), cyan("Motor disconnected "), blue("at "), cyan("port "), yellow(&port.to_string())],
                Log::MotorConnect(port) => colour_format![raw(&tick), cyan("Motor connected "), blue("at "), cyan("port "), yellow(&port.to_string())],
            });
        });
    }

    /// Gets the current state of the controller safely
    #[inline]
    pub fn controller<'a>(&'a mut self) -> Controller {
        Controller::new(self)
    }
}