use std::collections::VecDeque;
use std::time::Instant;
use crate::manager_w1::errors::W1Error;

mod errors;

/// Struct managing the 1-wire GPIO interface for thermometer sensor
///
pub struct W1Therm<'a> {
    name: &'a str,
    path: &'a str,
    ma: usize,
    threshold: f64,
    readings: VecDeque<f64>,
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
    /// * 'ma' - moving average history (zero or one means no moving average)
    /// * 'threshold' - threshold before change in temperature is reported
    pub fn new(name: &'a str, path: &'a str, ma: usize, threshold: f64) -> W1Therm<'a> {
        W1Therm { 
            name, 
            path, 
            ma: if ma == 0 { 1 } else { ma },
            threshold,
            readings: VecDeque::new(), 
            last_reading: 0.0, 
            last_report: Instant::now() 
        }
    }

    /// Performs a reading from the 1-wire interface and returns a temperature rounded
    /// to one decimal
    ///
    pub fn measure(&mut self) -> Result<(&'a str, Option<f64>), W1Error> {
        let data = std::fs::read_to_string(&self.path)?;
        let Some(t_pos) = data.find("t=") else {
            return Err(format!("corrupt w1 file: {}", data).into());
        };

        let temp = data[t_pos + 2..].trim().to_string().parse::<f64>()? / 1000.0;
        let avg = self.moving_average(to_one_decimal(temp));

        Ok((self.name, self.report(avg)))
    }
    
    /// Returns the sensor name
    /// 
    pub fn sensor(&self) -> &str {
        self.name
    }

    /// Calculates the moving average over the last `self.ma` readings
    /// 
    /// # Arguments
    /// 
    /// * 'temp' - temperature reading
    fn moving_average(&mut self, temp: f64) -> Option<f64> {
        self.readings.push_back(temp);
        
        if self.readings.len() < self.ma {
            return None;    
        }
        
        if self.readings.len() > self.ma {
            self.readings.pop_front();
        }
        
        let avg = to_one_decimal(self.readings.iter().sum::<f64>() / self.ma as f64);
        
        Some(avg)
    }
    
    /// Checks if a report shall be done and returns an option accordingly.
    /// 
    /// The policy is to report at least every 5 minute (300 secs) or
    /// whenever the last reading differs from previously reported one by 
    /// at least `self.threshold`.
    /// 
    /// # Arguments
    /// 
    /// * 'temp' - temperature reading to evaluate
    fn report(&mut self, temp: Option<f64>) -> Option<f64> {
        let Some(temp) = temp else { return None };
        
        if self.last_report.elapsed().as_secs() >= 300 || (self.last_reading - temp).abs() >= self.threshold {
            self.last_report = Instant::now();
            self.last_reading = temp;
            
            Some(temp)
        } else {
            None
        }
    }
}

/// Rounds the given value to one decimal
/// 
/// # Arguments
/// 
/// * 'input' - value to round
fn to_one_decimal(input: f64) -> f64 {
    (input * 10.0).round() / 10.0
}
