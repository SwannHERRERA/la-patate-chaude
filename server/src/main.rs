use std::net::{SocketAddr, TcpListener, TcpStream};
use std::io::Read;
use shared::Message;

fn main() {
  let addr = SocketAddr::from(([127, 0, 0, 1], 7878));
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
  println!("message_size: {}", decimal_size);

  // let mut bytes_of_message = Vec::<u8>::with_capacity(decimal_size as usize);

  // let _size_read = message.read_exact(&mut bytes_of_message);
  let mut bytes_of_message = Vec::<u8>::new();
  let _size_read = message.read_to_end(&mut bytes_of_message);
  println!("bytes_of_message length: {}", bytes_of_message.len());
  let message = String::from_utf8_lossy(&bytes_of_message);
  let message = serde_json::from_str(&message);
  println!("message={message:?}");
  match message {
    Ok(m) => m,
    Err(err) => panic!("Cannot parse message : {err:?}")
  }
}