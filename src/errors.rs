use std::fmt;
use std::fmt::Formatter;

/// Error representing an unrecoverable error that will halt the application
///
#[derive(Debug)]
pub struct UnrecoverableError(pub String);
impl fmt::Display for UnrecoverableError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "UnrecoverableError: {}", self.0)
    }
}
impl From<ConfigError> for UnrecoverableError {
    fn from(e: ConfigError) -> Self {
        UnrecoverableError(e.to_string())
    }
}
impl From<std::io::Error> for UnrecoverableError {
    fn from(e: std::io::Error) -> Self { UnrecoverableError(e.to_string()) }
}
impl From<core::net::AddrParseError> for UnrecoverableError {
    fn from(e: core::net::AddrParseError) -> Self { UnrecoverableError(e.to_string()) }
}

/// Errors while managing configuration
///
pub struct ConfigError(pub String);

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ConfigError: {}", self.0)
    }
}
impl From<std::io::Error> for ConfigError {
    fn from(err: std::io::Error) -> Self { ConfigError(err.to_string()) }
}
impl From<log::SetLoggerError> for ConfigError {
    fn from(e: log::SetLoggerError) -> Self {
        ConfigError(e.to_string())
    }
}
impl From<log4rs::config::runtime::ConfigErrors> for ConfigError {
    fn from(e: log4rs::config::runtime::ConfigErrors) -> Self {
        ConfigError(e.to_string())
    }
}
impl From<&str> for ConfigError {
    fn from(e: &str) -> Self { ConfigError(e.to_string()) }
}
impl From<String> for ConfigError {
    fn from(e: String) -> Self { ConfigError(e) }
}
