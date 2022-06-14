use std::net::{SocketAddr, TcpListener, TcpStream};
use std::io::Read;
use shared::config::{PORT, IP};
use shared::message::Message;

fn main() {
  let addr = SocketAddr::from((IP, PORT));
  let listener = TcpListener::bind(addr);

  let listener = match listener {
    Ok(l) => l,
    Err(err) => panic!("Cannot listen on port : {err:?}")
  };
  println!("Listening on : {}", addr);

  for message in listener.incoming() {
    println!("message={message:?}");
    let message = parse_message_from_tcp_stream(message.unwrap());

    println!("{message:?}");
  }
}

fn parse_message_from_tcp_stream(mut message: TcpStream) -> Message {
  let mut message_size = [0; 4];
  let _size_error = message.read(&mut message_size);
  let decimal_size = u32::from_be_bytes(message_size);

  let mut bytes_of_message = vec![0; decimal_size as usize];
  let _size_read = message.read_exact(&mut bytes_of_message);
  let message = String::from_utf8_lossy(&bytes_of_message);
  let message = serde_json::from_str(&message);
  match message {
    Ok(m) => m,
    Err(err) => panic!("Cannot parse message : {err:?}")
  }
}