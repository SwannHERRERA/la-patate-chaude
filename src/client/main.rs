mod message;

use std::{net::TcpStream, io::Write};
use message::Message;


const IP: &'static str = "127.0.0.1";
const PORT: u16 = 7878;

fn main() {
    let address = format!("{}:{}", IP, PORT);
    match TcpStream::connect(address) {
        Ok(mut stream) => {
            if let Ok(message) = serde_json::to_string(&Message::Subscribe { name: "Swann".to_string() }) {
                let result = stream.write(message.as_bytes()).unwrap();
                println!("result : {}, message: {}", result, message);
                // match result {
                //     Ok(_) => println!("Successfully wrote to stream"),
                //     Err(e) => println!("Error: {}", e),
                // }
            }

            

        },
        Err(_) => panic!("Could not connect to server {} on port {}", IP, PORT),
    }
}