use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener};
use std::sync::{Arc, Mutex};
use std::thread;
use log::{error, info};
use crate::errors::UnrecoverableError;
use crate::initialization::config;
use crate::manager_w1::run;

mod errors;
mod initialization;
mod logging;
mod manager_w1;

const HTTP_RESPONSE: &str = "HTTP/1.1 200 OK\r\n\r\n";

fn main() -> Result<(), UnrecoverableError> {
    let config = config()?;

    let temperature: Arc<Mutex<f64>> = Arc::new(Mutex::new(0.0));
    let c_temperature = temperature.clone();
    
    thread::spawn(move || {
        run(c_temperature, &config.sensor_w1.path, config.sensor_w1.ma_window, config.sensor_w1.threshold)
    });
    

    let socket_addr = SocketAddr::new(config.web_server.bind_address.parse()?, config.web_server.bind_port);
    let listener = TcpListener::bind(socket_addr)?;
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0; 1024];
                match stream.read(&mut buffer) {
                    Ok(_) => {
                        let request = String::from_utf8_lossy(&buffer[..]);
                        if request.starts_with("GET /read ") || request.starts_with("GET /read/ ") {
                            info!("got read request");
                            let data: f64;
                            {
                                data = *temperature.lock().unwrap();
                            }
                            if let Err(e) = stream.write(http_response(data).as_bytes()) {
                                error!("could not write to stream: {}", e);
                            }
                        }
                    },
                    Err(e) => { error!("failed to read from stream: {}", e); }
                }
            },
            Err(e) => { error!("failed to get stream for requestor: {}", e); }
        }
    }

    Ok(())
}

/// Creates an HTTP response string with data in json
///
/// # Arguments
///
/// * 'data' - data to include in response
fn http_response(data: f64) -> String {
    format!("{}{{\"data\": {}}}", HTTP_RESPONSE, data)
}