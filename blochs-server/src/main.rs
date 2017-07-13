#[macro_use]
extern crate log;
extern crate simple_logger;
extern crate libblochs;

use std::net::{TcpListener, TcpStream};
use std::io::Read;
use libblochs::config::load_server_config;

fn main() {
    simple_logger::init().unwrap();

    let config = load_server_config();

    info!("Starting blochs server listening on {}", config.listening_port.unwrap());

    let listener = TcpListener::bind(format!("127.0.0.1:{}", config.listening_port.unwrap())).unwrap();

    fn handle_client(stream: TcpStream) {
        let mut message = String::new();
        match stream.try_clone() {
            Ok(mut copy) => {
                match copy.read_to_string(&mut message) {
                    Ok(_) => info!("Received message: {}", message),
                    Err(reason) => error!("Failed to read content, {}", reason)
                }
            },
            Err(reason) => {
                error!("Broken connection {}", reason);
            }
        }
    }

    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(reason) => {
                error!("Broken connection {}", reason);
            }
        }
    }
}
