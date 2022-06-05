use std::{net::TcpStream, io::Write};
use shared::Message;


const IP: &'static str = "127.0.0.1";
const PORT: u16 = 7878;

fn main() {
    let address = format!("{}:{}", IP, PORT);
    match TcpStream::connect(address) {
        Ok(stream) => {
            let message = Message::Subscribe { name: "test".to_string() };
            send_message(stream, message);
        },
        Err(_) => panic!("Could not connect to server {} on port {}", IP, PORT),
    }
}

fn send_message(mut stream: TcpStream, message: Message) {
    if let Ok(message) = serde_json::to_string(&message) {
        let result = stream.write(message.as_bytes()).unwrap();
        println!("result : {}, message: {}", result, message);
    } 
}