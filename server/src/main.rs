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
    let mut v = Vec::<u8>::new();
    let _size_read = message.read_to_end(&mut v);
    let message = String::from_utf8_lossy(&v);
    let message = serde_json::from_str(&message);
    match message {
      Ok(m) => m,
      Err(err) => panic!("Cannot parse message : {err:?}")
    }
}