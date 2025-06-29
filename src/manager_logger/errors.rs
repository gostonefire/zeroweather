use std::fmt;
use std::fmt::Formatter;


/// Error representing error that occurs while communicating with the logger
///
pub struct WeatherLoggerError(pub String);
impl fmt::Display for WeatherLoggerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "WeatherLoggerError: {}", self.0)
    }
}
impl From<ureq::Error> for WeatherLoggerError {
    fn from(e: ureq::Error) -> Self { WeatherLoggerError(e.to_string()) }
}
impl From<String> for WeatherLoggerError {
    fn from(e: String) -> Self { WeatherLoggerError(e) }
}