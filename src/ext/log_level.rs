//! Conversions for [`LogLevel`].

use crate::xterm::LogLevel;

use log::Level;

impl From<Level> for LogLevel {
    fn from(level: Level) -> LogLevel {
        match level {
            Level::Trace | Level::Debug => LogLevel::Debug,
            Level::Info => LogLevel::Info,
            Level::Warn => LogLevel::Warn,
            Level::Error => LogLevel::Error,
        }
    }
}

#[derive(Debug, Clone)]
#[allow(clippy::module_name_repetitions)]
/// Type indicating that a [`LogLevel`] to [`Level`] conversion failed because
/// the [`LogLevel`] was [`Off`](LogLevel::Off).
pub struct LogLevelIsOff;

impl std::convert::TryFrom<LogLevel> for Level {
    type Error = LogLevelIsOff;

    fn try_from(level: LogLevel) -> Result<Level, LogLevelIsOff> {
        match level {
            LogLevel::Debug => Ok(Level::Debug),
            LogLevel::Info => Ok(Level::Info),
            LogLevel::Warn => Ok(Level::Warn),
            LogLevel::Error => Ok(Level::Error),
            LogLevel::Off => Err(LogLevelIsOff),
            _ => unreachable!(),
        }
    }
}
