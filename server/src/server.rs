use crate::message_handler::MessageHandler;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::io::{Read, Write};
use log::{info, debug};
use shared::config::{PORT, IP};
use shared::message::Message;



pub struct Server {
  listener: TcpListener,
  message_handler: MessageHandler,
}

impl Server {
  pub fn new(listener: TcpListener, message_handler: MessageHandler) -> Server {
    Server { listener, message_handler }
  }

  pub fn listen(&mut self) {
    for message in self.listener.incoming() {
      debug!("message={message:?}");
      let tcp_stream = message.unwrap();
      let parsed_message = self.parse_message_from_tcp_stream(&tcp_stream);
      let response = self.message_handler.handle_message(parsed_message);
      self.send_response(response, &tcp_stream);
    }
  }

  fn parse_message_from_tcp_stream(&self, mut message: &TcpStream) -> Message {
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

  fn send_response(&self, response: Message, mut tcp_stream: &TcpStream) {
    let response = serde_json::to_string(&response).unwrap();
    let response = response.as_bytes();
    let response_size = response.len() as u32;
    let response_length_as_bytes = response_size.to_be_bytes();
    let result = tcp_stream.write(&[&response_length_as_bytes, response].concat());
    info!("byte write : {:?}, ", result);
  }
}



pub fn create_listener() -> TcpListener {
  let addr = SocketAddr::from((IP, PORT));
  let listener = TcpListener::bind(addr);
  info!("Start Listening on : {}", addr);
  match listener {
    Ok(l) => l,
    Err(err) => panic!("Cannot listen on port : {err:?}")
  }
}