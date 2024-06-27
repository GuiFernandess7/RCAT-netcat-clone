use std::fmt::format;
use std::fs::read;
use std::io::{Read, stdout, Write};
use std::net::TcpListener;

pub fn run(bind_host: &String, port: &u16) -> Result<(), String> {
    let addr = format!("{}:{}", bind_host, port);
    let listener = TcpListener::bind(addr.clone()).map_err(|_| format!("Failed to bind to {}", addr))?;

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                println!("Connection accepted");
            }
            Err(e) => {
                println!("Error while accepting incoming connection - {}", e);
            }
        }
    }

    Ok(())
}