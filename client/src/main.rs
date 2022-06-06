use std::{net::TcpStream, io::Write};
use shared::Message;


const IP: &'static str = "127.0.0.1";
const PORT: u16 = 7878;

fn main() {
    let address = format!("{}:{}", IP, PORT);
    match TcpStream::connect(address) {
        Ok(stream) => {
            let message = Message::Subscribe { name: "tesa;ihilah;iuvbasd;iuv'ouwdnwbdvkjbwdjkvbijwdkhbvi;bwDKVBJKWDBVIHWIOUFHIWHBFIHWFOHIWUHFOHWFOUJWEOFHIOWEJFOIJwbdvkjbwdjkvbijwdkhbvi;bwDKVBJKWDBVIHWIOUFHIWHBFIHWFOHIWUHFOHWFOUJWEOFHIOWEJFOIJwbdvkjbwdjkvbijwdkhbvi;bwDKVBJKWDBVIHWIOUFHIWHBFIHWFOHIWUHFOHWFOUJWEOFHIOWEJFOIJwbdvkjbwdjkvbijwdkhbvi;bwDKVBJKWDBVIHWIOUFHIWHBFIHWFOHIWUHFOHWFOUJWEOFHIOWEJFOIJwbdvkjbwdjkvbijwdkhbvi;bwDKVBJKWDBVIHWIOUFHIWHBFIHWFOHIWUHFOHWFOUJWEOFHIOWEJFOIJwbdvkjbwdjkvbijwdkhbvi;bwDKVBJKWDBVIHWIOUFHIWHBFIHWFOHIWUHFOHWFOUJWEOFHIOWEJFOIJwbdvkjbwdjkvbijwdkhbvi;bwDKVBJKWDBVIHWIOUFHIWHBFIHWFOHIWUHFOHWFOUJWEOFHIOWEJFOIJwbdvkjbwdjkvbijwdkhbvi;bwDKVBJKWDBVIHWIOUFHIWHBFIHWFOHIWUHFOHWFOUJWEOFHIOWEJFOIJvk;wbdvkjbwdjkvbijwdkhbvi;bwDKVBJKWDBVIHWIOUFHIWHBFIHWFOHIWUHFOHWFOUJWEOFHIOWEJFOIJt".to_string() };
            send_message(stream, message);
        },
        Err(_) => panic!("Could not connect to server {} on port {}", IP, PORT),
    }
}

fn send_message(mut stream: TcpStream, message: Message) {
    if let Ok(message) = serde_json::to_string(&message) {
        let bytes_message = message.as_bytes();
        let message_size = bytes_message.len() as u32;
        let message_length_as_bytes = message_size.to_be_bytes();
        stream.write(&message_length_as_bytes).unwrap();
        let result = stream.write(bytes_message).unwrap();
        println!("result : {}, message: {}", result, message);
    } 
}