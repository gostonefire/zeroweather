use std::time::Instant;
use crate::manager_w1::errors::W1Error;

mod errors;

/// Struct managing the 1-wire GPIO interface for thermometer sensor
///
pub struct W1Therm<'a> {
    name: &'a str,
    path: &'a str,
    last_reading: f64,
    last_report: Instant,
}

impl<'a> W1Therm<'a> {

    /// Creates a new W1Therm instance
    ///
    /// # Arguments
    ///
    /// * 'name' - the name of the sensor
    /// * 'path' - the path to the bus file carrying (and triggering) the measurement
    pub fn new(name: &'a str, path: &'a str) -> W1Therm<'a> {
        W1Therm { name, path, last_reading: 0.0, last_report: Instant::now() }
    }

    /// Performs a reading from the 1-wire interface and returns a temperature rounded
    /// to one decimal
    ///
    pub fn measure(&mut self) -> Result<(&'a str, Option<f64>), W1Error> {
        let data = std::fs::read_to_string(&self.path)?;
        let Some(t_pos) = data.find("t=") else {
            return Err(format!("corrupt w1 file: {}", data).into());
        };

        let temp = data[t_pos + 2..].trim().to_string().parse::<f64>()?;

        Ok((self.name, self.report((temp / 100.0).round() / 10.0)))
    }
    
    /// Returns the sensor name
    /// 
    pub fn sensor(&self) -> &str {
        self.name
    }

    /// Checks if a report shall be done and returns an option accordingly.
    /// 
    /// The policy is to report at least every 5 minute (300 secs) or
    /// whenever the last reading differs from previously reported one.
    /// 
    /// # Arguments
    /// 
    /// * 'temp' - temperature reading to evaluate
    fn report(&mut self, temp: f64) -> Option<f64> {

        if self.last_report.elapsed().as_secs() >= 300 || self.last_reading != temp {
            self.last_report = Instant::now();
            self.last_reading = temp;
            
            Some(temp)
        } else {
            None
        }
    }
}