use std::time::Duration;
use ureq::Agent;
use crate::manager_logger::errors::WeatherLoggerError;

mod errors;

/// Struct for managing communication with the weather logger
///
pub struct WeatherLogger<'a> {
    agent: Agent,
    url: &'a str,
}

impl<'a> WeatherLogger<'a> {

    /// Creates a new instance of WeatherLogger
    ///
    /// # Arguments
    ///
    /// * 'url' - url to the logger
    pub fn new(url: &'a str) -> Self {
        let agent_config = Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(30)))
            .build();

        let agent = agent_config.into();

        Self { agent, url }
    }

    pub fn report(&mut self, id: &str, temp: Option<f64>) -> Result<(), WeatherLoggerError> {
        // If none we just return Ok
        let Some(t) = temp else {
            return  Ok(())
        };

        // Build query parameters
        let t_string = t.to_string();
        let query = vec![
            ("id", id),
            ("temp", &t_string),
            ("hum", "0")
        ];

        // Send report to logger
        let response = self.agent
            .get(self.url)
            .query_pairs(query)
            .call()?;
        
        if response.status() != 200 {
            return Err(response.status().to_string().into());
        }
        Ok(())
    }
}