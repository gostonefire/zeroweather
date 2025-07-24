use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use log::{debug, error, info};
use crate::manager_w1::errors::W1Error;

mod errors;

/// Runs the sensor measuring loop and updates the temperature mutex value
///
/// Although a temperature change below given threshold is not reported, the mutex is
/// updated after each measure anyway since the number of requestors is only one.
///
/// # Arguments
///
/// * 'temperature' - temperature mutex to update
/// * 'path' - the path to the bus file carrying (and triggering) the measurement
/// * 'ma' - moving average history (zero or one means no moving average)
/// * 'threshold' - threshold before change in temperature is reported
pub fn run(temperature: Arc<Mutex<f64>>, path: &str, ma: usize, threshold: f64) {
    info!("starting measuring loop");
    let mut w1_therm = W1Therm::new(path, ma, threshold);

    loop {
        match w1_therm.measure() {
            Ok(_) => {
                *temperature.lock().unwrap() = w1_therm.report();
            },
            Err(e) => { error!("error while reading from sensor: {}", e) },
        }

        thread::sleep(Duration::from_secs(60));
    }
}


/// Struct managing the 1-wire GPIO interface for thermometer sensor
///
struct W1Therm<'a> {
    path: &'a str,
    ma: usize,
    threshold: f64,
    readings: VecDeque<f64>,
    last_report: f64,
    last_average: f64,
}

impl<'a> W1Therm<'a> {

    /// Creates a new W1Therm instance
    ///
    /// # Arguments
    ///
    /// * 'path' - the path to the bus file carrying (and triggering) the measurement
    /// * 'ma' - moving average history (zero or one means no moving average)
    /// * 'threshold' - threshold before change in temperature is reported
    fn new(path: &'a str, ma: usize, threshold: f64) -> W1Therm<'a> {
        W1Therm { 
            path,
            ma: if ma == 0 { 1 } else { ma },
            threshold,
            readings: VecDeque::new(), 
            last_report: 0.0,
            last_average: 0.0,
        }
    }

    /// Performs a reading from the 1-wire interface and returns a temperature rounded
    /// to one decimal
    ///
    fn measure(&mut self) -> Result<(), W1Error> {
        let data = std::fs::read_to_string(&self.path)?;
        let Some(t_pos) = data.find("t=") else {
            return Err(format!("corrupt w1 file: {}", data).into());
        };

        let temp = data[t_pos + 2..].trim().to_string().parse::<f64>()? / 1000.0;
        self.last_average = self.moving_average(to_one_decimal(temp));

        Ok(())
    }
    
    /// Calculates the moving average over the last `self.ma` readings
    /// 
    /// # Arguments
    /// 
    /// * 'temp' - temperature reading
    fn moving_average(&mut self, temp: f64) -> f64 {
        self.readings.push_back(temp);
        
        if self.readings.len() > self.ma {
            self.readings.pop_front();
        }
        
        to_one_decimal(self.readings.iter().sum::<f64>() / self.readings.len() as f64)
    }
    
    /// Reports current temperature
    ///
    /// If the current moving average is still within threshold, then the last reported value
    /// is returned again.
    /// 
    fn report(&mut self) -> f64 {
        if (self.last_report - self.last_average).abs() >= self.threshold {
            debug!("updating report value from {} to {}", self.last_report, self.last_average);
            self.last_report = self.last_average;
        }

        self.last_report
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
