use std::thread;
use std::time::Duration;
use log::{error, info};
use crate::errors::UnrecoverableError;
use crate::initialization::config;
use crate::manager_logger::WeatherLogger;
use crate::manager_w1::W1Therm;

mod errors;
mod initialization;
mod logging;
mod manager_w1;
mod manager_logger;

fn main() -> Result<(), UnrecoverableError> {
    let config = config()?;
    let mut w1_therm = W1Therm::new(&config.sensor_w1.thermometer.0, &config.sensor_w1.thermometer.1);
    let mut weather_logger = WeatherLogger::new(&config.weatherlogger.url);
    
    loop {
        match w1_therm.measure() {
            Ok((sensor, temp)) => { 
                info!("sensor \"{}\" measured: {:?}", sensor, temp);
                if let Err(e) = weather_logger.report(sensor, temp) {
                    error!("error while reporting sensor \"{}\" to logger: {}", sensor, e);
                }
            }
            Err(e) => { error!("error while measure from sensor \"{}\": {}", w1_therm.sensor(), e); }
        }
        
        thread::sleep(Duration::from_secs(60));
    }
}
