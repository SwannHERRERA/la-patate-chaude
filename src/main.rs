mod message;

use std::{net::TcpStream, io::Write};


const IP: &'static str = "127.0.0.1";
const PORT: u16 = 7878;

fn main() {
    let address = format!("{}:{}", IP, PORT);
    match TcpStream::connect(address) {
        Ok(mut stream) => {
            stream.write(b"Hello, world!");
        },
        Err(_) => panic!("Could not connect to server {} on port {}", IP, PORT),
    }
}