use std::{net::TcpStream, io::Write};
use shared::message::Message;
use shared::config::{IP, PORT};

fn main() {
    let ip_as_string = IP.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(".");
    let address = format!("{}:{}", ip_as_string, PORT);
    match TcpStream::connect(address) {
        Ok(stream) => {
            let message = Message::Subscribe { name: "test".to_string() };
            send_message(stream, message);
        },
        Err(_) => panic!("Could not connect to server {:?} on port {}", IP, PORT),
    }
}

fn send_message(mut stream: TcpStream, message: Message) {
    if let Ok(message) = serde_json::to_string(&message) {
        let result = stream.write(message.as_bytes()).unwrap();
        println!("result : {}, message: {}", result, message);
    }
}