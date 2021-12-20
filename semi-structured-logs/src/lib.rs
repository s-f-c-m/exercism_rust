// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug,
}

use std::fmt::{Display, Formatter, Result};

impl Display for LogLevel {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            LogLevel::Info => {
                write!(f, "[INFO]: ")
            }
            LogLevel::Warning => {
                write!(f, "[WARNING]: ")
            }
            LogLevel::Error => {
                write!(f, "[ERROR]: ")
            }
            LogLevel::Debug => {
                write!(f, "[DEBUG]: ")
            }
        }
    }
}

/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    level.to_string() + message
    // unimplemented!()
}
pub fn info(message: &str) -> String {
    // unimplemented!()
    log(LogLevel::Info, &message)
}
pub fn warn(message: &str) -> String {
    // unimplemented!()
    log(LogLevel::Warning, &message)
}
pub fn error(message: &str) -> String {
    // unimplemented!()
    log(LogLevel::Error, &message)
}
