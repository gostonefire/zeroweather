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
impl From<toml::de::Error> for ConfigError {
    fn from(e: toml::de::Error) -> Self {
        ConfigError(e.to_string())
    }
}
