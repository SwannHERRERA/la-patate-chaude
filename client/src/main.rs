use std::{io::{Read, Write}, net::TcpStream};
use std::borrow::Cow;

use rand;
use rand::Rng;

use shared::Message;

const IP: &'static str = "127.0.0.1";
const PORT: u16 = 7878;

fn main() {
    let address = format!("{}:{}", IP, PORT);
    match TcpStream::connect(address) {
        Ok(stream) => {
            let message = Message::Hello;
            send_message(&stream, message);
            receive_messages(&stream);
        },
        Err(_) => panic!("Could not connect to server {} on port {}", IP, PORT),
    }
}

fn send_message(mut stream: &TcpStream, message: Message) {
    if let Ok(message) = serde_json::to_string(&message) {
        let bytes_message = message.as_bytes();
        let message_size = bytes_message.len() as u32;
        let message_length_as_bytes = message_size.to_be_bytes();
        let result = stream.write(&[&message_length_as_bytes, bytes_message].concat());
        println!("result : {:?}, message: {}", result, message);
    }
}

fn receive_messages(mut stream: &TcpStream){
    loop {
        let mut buf_size = [0; 4];
        stream.read(&mut buf_size);
        let res_size = u32::from_be_bytes(buf_size);
        if res_size == 0 {
            continue
        }

        let mut buf = vec![0; res_size as usize];
        stream.read(&mut buf);
        let str = String::from_utf8_lossy(&buf);
        println!(": {:?}", str);

        match serde_json::from_str(&str) {
            Ok(message) => dispatch_messages(stream, message),
            Err(_) => println!("Error while parsing message"),
        }
    }
}

fn dispatch_messages(mut stream: &TcpStream, message: Message) {
    println!("Dispatching: {:?}", message);
    match message {
        Message::Welcome { version } => {
            let mut rng = rand::thread_rng();
            let n1: u8 = rng.gen();
            let answer = Message::Subscribe { name: "test".to_string()+ &*n1.to_string() };
            send_message(&stream, answer);
        }
        _ => {}
    }
}
