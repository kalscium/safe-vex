use alloc::string::String;
use crate::pile::Pile;

pub type Logger = Pile<Log>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Log {
    MotorConnect(u8),
    MotorError(u8),
    Other(String),
}