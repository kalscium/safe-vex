use alloc::string::String;
use crate::pile::Pile;

/// Uses a pile for logs
pub type Logger = Pile<Log>;

/// Different types of logs that the robot may report
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Log {
    MotorConnect(u8),
    MotorDisconnect(u8),
    ControllerDisconnect,
    ControllerConnect,
    Autonomous,
    OpControl,
    Disabled,
    Nothing,
    Other(String),
}