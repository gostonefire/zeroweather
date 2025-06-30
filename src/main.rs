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
    let mut w1_sensors: Vec<W1Therm> = Vec::new();
    for s in config.sensor_w1.thermometer.iter() {
        w1_sensors.push(W1Therm::new(&s.0, &s.1, s.2))
    }

    let mut weather_logger = WeatherLogger::new(&config.weatherlogger.url);
    
    loop {
        for w1 in w1_sensors.iter_mut() {
            
            match w1.measure() {
                Ok((sensor, temp)) => { 
                    info!("sensor \"{}\" measured: {:?}", sensor, temp);
                    if let Err(e) = weather_logger.report(sensor, temp) {
                        error!("error while reporting sensor \"{}\" to logger: {}", sensor, e);
                    }
                }
                Err(e) => { error!("error while measure from sensor \"{}\": {}", w1.sensor(), e); }
            }
        }
        
        thread::sleep(Duration::from_secs(60));
    }
}
