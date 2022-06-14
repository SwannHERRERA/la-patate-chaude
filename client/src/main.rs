use std::{io::{Read, Write}, net::TcpStream};
use shared::config::{IP, PORT};

use rand;
use rand::Rng;

use shared::message::Message;

fn main() {
    let ip_as_string = IP.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(".");
    let address = format!("{}:{}", ip_as_string, PORT);
    match TcpStream::connect(address) {
        Ok(stream) => {
            let message = Message::Hello;
            send_message(&stream, message);
            receive_messages(&stream);
        },
        Err(_) => panic!("Could not connect to server {:?} on port {}", IP, PORT),
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
        let string_receive = String::from_utf8_lossy(&buf);
        println!(": {:?}", string_receive);

        match serde_json::from_str(&string_receive) {
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
