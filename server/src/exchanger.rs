use std::{sync::{Arc, Mutex}, net::TcpStream, io::{Error, Read, Write}};

use log::trace;
use shared::message::Message;

use crate::message_handler::MessageHandler;

pub struct Exchanger {
  message_handler: Arc<Mutex<MessageHandler>>,
}

impl Exchanger {

  pub fn new(message_handler: Arc<Mutex<MessageHandler>>) -> Exchanger {
    Exchanger { message_handler }
  }

  pub fn hold_communcation(&mut self, stream: Result<TcpStream, Error>) {
    let tcp_stream = stream.unwrap();
    let mut message_handler = self.message_handler.lock().unwrap();
    let parsed_message = self.parse_message_from_tcp_stream(&tcp_stream);
    let response = message_handler.handle_message(parsed_message);
    self.send_response(response, &tcp_stream);
    drop(message_handler);
    loop {
      let parsed_message = self.parse_message_from_tcp_stream(&tcp_stream);
      let mut message_handler = self.message_handler.lock().unwrap();
      let response = message_handler.handle_message(parsed_message);
      drop(message_handler);
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
    trace!("byte write : {:?}, ", result);
  }
}

