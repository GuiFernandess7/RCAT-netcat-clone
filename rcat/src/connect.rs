use std::{fmt::format, io::Write, net::{TcpListener, TcpStream}};

pub fn run(host: &String, port: &u16) -> Result<(), String> {
    let addr = format!("{}:{}", host, port);
    let mut client = TcpStream::connect(addr.as_str()).map_err(|_| format!("Failed to connect to {}", addr))?;
    client.write("hello, TCP".as_bytes()).map_err(|_| format!("Failed to send"))?;
    Ok(())
}