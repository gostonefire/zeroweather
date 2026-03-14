use std::{env, fs};
use log::LevelFilter;
use crate::errors::ConfigError;
use crate::logging::setup_logger;


/// Configuration parameters for the web server
///
/// # Arguments
///
/// * 'bind_address' - the address for the web server to bind to
/// * 'bind_port' - the port for the web server to bind to
pub struct WebServerParameters {
    pub bind_address: String,
    pub bind_port: u16,
}

/// Configuration parameters for the 1-wire sensor
///
/// # Arguments
///
/// * 'path' - the path to the bus file carrying (and triggering) the measurement
/// * 'ma_window' - moving average history (zero or one means no moving average)
/// * 'threshold' - threshold before change in temperature is reported
pub struct SensorW1 {
    pub path: String,
    pub ma_window: usize,
    pub threshold: f64,
}

/// General configuration parameters for the application
///
/// # Arguments
///
/// * 'log_path' - path to the log file
/// * 'log_level' - logging level (Off, Error, Warn, Info, Debug, Trace)
/// * 'log_to_stdout' - if true, logging is also written to stdout
pub struct General {
    pub log_path: String,
    pub log_level: LevelFilter,
    pub log_to_stdout: bool,
}

/// The overall configuration for the application
///
pub struct Config {
    pub web_server: WebServerParameters,
    pub sensor_w1: SensorW1,
    pub general: General,
}

/// Struct used during the parsing of the configuration file
///
/// It holds optional values for all configuration items
struct PartialConfig {
    web_server_bind_address: Option<String>,
    web_server_bind_port: Option<u16>,
    sensor_w1_path: Option<String>,
    sensor_w1_ma_window: Option<usize>,
    sensor_w1_threshold: Option<f64>,
    general_log_path: Option<String>,
    general_log_level: Option<LevelFilter>,
    general_log_to_stdout: Option<bool>,
}

impl PartialConfig {
    /// Creates a new PartialConfig instance with all values set to None
    ///
    fn new() -> Self {
        Self {
            web_server_bind_address: None,
            web_server_bind_port: None,
            sensor_w1_path: None,
            sensor_w1_ma_window: None,
            sensor_w1_threshold: None,
            general_log_path: None,
            general_log_level: None,
            general_log_to_stdout: None,
        }
    }

    /// Builds a Config struct from the PartialConfig instance
    ///
    /// Returns an error if any of the required configuration items are missing
    fn build(self) -> Result<Config, ConfigError> {
        Ok(Config {
            web_server: WebServerParameters {
                bind_address: Self::require(self.web_server_bind_address, "web_server.bind_address")?,
                bind_port: Self::require(self.web_server_bind_port, "web_server.bind_port")?,
            },
            sensor_w1: SensorW1 {
                path: Self::require(self.sensor_w1_path, "sensor_w1.path")?,
                ma_window: Self::require(self.sensor_w1_ma_window, "sensor_w1.ma_window")?,
                threshold: Self::require(self.sensor_w1_threshold, "sensor_w1.threshold")?,
            },
            general: General {
                log_path: Self::require(self.general_log_path, "general.log_path")?,
                log_level: Self::require(self.general_log_level, "general.log_level")?,
                log_to_stdout: Self::require(self.general_log_to_stdout, "general.log_to_stdout")?,
            },
        })
    }

    /// Helper function to require an optional value and return an error if it's None
    ///
    /// # Arguments
    ///
    /// * 'value' - the optional value to check
    /// * 'key' - the configuration key associated with the value
    fn require<T>(value: Option<T>, key: &str) -> Result<T, ConfigError> {
        value.ok_or_else(|| ConfigError::from(format!("missing config key: {}", key)))
    }
}

/// Returns a configuration struct for the application and starts logging
///
pub fn config() -> Result<Config, ConfigError> {
    let args: Vec<String> = env::args().collect();
    let config_path = args.iter()
        .find(|p| p.starts_with("--config="))
        .ok_or(ConfigError::from("missing --config=<config_path>"))?;
    let config_path = config_path
        .split_once('=')
        .ok_or(ConfigError::from("invalid --config=<config_path>"))?
        .1;

    let config = load_config(config_path)?;

    setup_logger(
        &config.general.log_path,
        config.general.log_level,
        config.general.log_to_stdout,
    )?;

    Ok(config)
}

/// Loads the configuration file and returns a struct with all configuration items
///
/// # Arguments
///
/// * 'config_path' - path to the configuration file
fn load_config(config_path: &str) -> Result<Config, ConfigError> {
    let text = fs::read_to_string(config_path)?;
    parse_config(&text)
}

/// Parses the configuration text and returns a Config struct
///
/// # Arguments
///
/// * 'text' - the configuration text to parse
fn parse_config(text: &str) -> Result<Config, ConfigError> {
    let mut partial = PartialConfig::new();

    for (index, raw_line) in text.lines().enumerate() {
        let line_number = index + 1;
        let line = raw_line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let (key, value) = line
            .split_once('=')
            .ok_or_else(|| ConfigError::from(format!("line {}: expected key=value", line_number)))?;

        let key = key.trim();
        let value = value.trim();

        match key {
            "web_server.bind_address" => {
                partial.web_server_bind_address = Some(value.to_string());
            }
            "web_server.bind_port" => {
                partial.web_server_bind_port = Some(parse_value(value, key, line_number)?);
            }
            "sensor_w1.path" => {
                partial.sensor_w1_path = Some(value.to_string());
            }
            "sensor_w1.ma_window" => {
                partial.sensor_w1_ma_window = Some(parse_value(value, key, line_number)?);
            }
            "sensor_w1.threshold" => {
                partial.sensor_w1_threshold = Some(parse_value(value, key, line_number)?);
            }
            "general.log_path" => {
                partial.general_log_path = Some(value.to_string());
            }
            "general.log_level" => {
                partial.general_log_level = Some(parse_log_level(value, line_number)?);
            }
            "general.log_to_stdout" => {
                partial.general_log_to_stdout = Some(parse_value(value, key, line_number)?);
            }
            _ => {
                return Err(ConfigError::from(format!(
                    "line {}: unknown config key: {}",
                    line_number, key
                )));
            }
        }
    }

    partial.build()
}

/// Helper function to parse a value from a string
///
/// # Arguments
///
/// * 'value' - the string value to parse
/// * 'key' - the configuration key associated with the value
/// * 'line_number' - the line number in the configuration file
fn parse_value<T>(value: &str, key: &str, line_number: usize) -> Result<T, ConfigError>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    value.parse::<T>().map_err(|e| {
        ConfigError::from(format!(
            "line {}: invalid value for {}: {}",
            line_number, key, e
        ))
    })
}

/// Helper function to parse a log level from a string
///
/// # Arguments
///
/// * 'value' - the string value to parse
/// * 'line_number' - the line number in the configuration file
fn parse_log_level(value: &str, line_number: usize) -> Result<LevelFilter, ConfigError> {
    match value {
        "Off" => Ok(LevelFilter::Off),
        "Error" => Ok(LevelFilter::Error),
        "Warn" => Ok(LevelFilter::Warn),
        "Info" => Ok(LevelFilter::Info),
        "Debug" => Ok(LevelFilter::Debug),
        "Trace" => Ok(LevelFilter::Trace),
        _ => Err(ConfigError::from(format!(
            "line {}: invalid value for general.log_level: {}",
            line_number, value
        ))),
    }
}
