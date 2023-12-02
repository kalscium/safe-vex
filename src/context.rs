use alloc::string::ToString;
use vex_rt::{prelude::Peripherals, io::println, rtos::Mutex};
use crate::{log::{Logger, Log}, controller::Controller, colour_format, bot::TICK_SPEED};

pub type TickType = u16;

/// The context (current state) of the robot's execution
pub struct Context {
    pub(crate) perph: Peripherals,
    logger: Mutex<Logger>,
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
            logger: Mutex::new(Logger::new()),
            is_controller_disconnected: false,
            tick: 0,
        }
    }

    /// Logs a log to the context's log pile
    #[inline]
    pub fn log(&self, log: Log) {
        self.logger.lock().push(log);
    }

    /// Wipes the logs in the log pile and also prints them to the console screen
    #[inline]
    pub fn flush_logs(&mut self) {
        let mut tick = 0u32;
        println!("{}", colour_format![blue("\n==="), cyan(" Context  Robot Logs "), blue("===")]);
        self.logger.lock().flush(|log, i| {
            let tick_str = colour_format![blue("( "), yellow(&(tick as f64 * TICK_SPEED as f64 /1000f64).to_string()), yellow("s"), blue(" ) ")];
            println!("{}", match log {
                Log::Other(x) => colour_format![raw(&tick_str), cyan("Custom Log"), blue(": "), none(x)],
                Log::Autonomous => colour_format![raw(&tick_str), cyan("Autonomous period "), green("started")],
                Log::OpControl => colour_format![raw(&tick_str), cyan("Driver Control period "), green("started")],
                Log::Disabled => colour_format![raw(&tick_str), cyan("Disabled period "), green("started")],
                Log::ControllerDisconnect => colour_format![raw(&tick_str), cyan("Controller "), red("disconnect")],
                Log::ControllerConnect => colour_format![raw(&tick_str), cyan("Controller "), green("connected")],
                Log::MotorDisconnect(port) => colour_format![raw(&tick_str), cyan("Motor disconnected "), blue("at "), cyan("port "), yellow(&port.to_string())],
                Log::MotorConnect(port) => colour_format![raw(&tick_str), cyan("Motor connected "), blue("at "), cyan("port "), yellow(&port.to_string())],
                Log::Nothing => return tick += i as u32,
            });
            tick += i as u32;
        });
    }

    /// Gets the current state of the controller safely
    #[inline]
    pub fn controller(&mut self) -> Controller {
        Controller::new(self)
    }
}