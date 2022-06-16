use std::net::{SocketAddr, TcpListener, TcpStream};
use std::io::Read;
use log::{info, debug};
use message_handler::MessageHandler;
use shared::config::{PORT, IP};
use shared::message::Message;

mod message_handler;


struct Server {
    listener: TcpListener,
    message_handler: MessageHandler,
}

impl Server {
  fn new(listener: TcpListener, message_handler: MessageHandler) -> Server {
    Server { listener, message_handler }
  }

  fn listen(&mut self) {
    for message in self.listener.incoming() {
      debug!("message={message:?}");
      let message = self.parse_message_from_tcp_stream(message.unwrap());
      let _response = self.message_handler.handle_message(message);
      // TODO response
    }
  }

  fn parse_message_from_tcp_stream(&self, mut message: TcpStream) -> Message {
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
}


fn create_listener() -> TcpListener {
  let addr = SocketAddr::from((IP, PORT));
  let listener = TcpListener::bind(addr);
  info!("Start Listening on : {}", addr);
  match listener {
    Ok(l) => l,
    Err(err) => panic!("Cannot listen on port : {err:?}")
  }
}

fn main() {
  std::env::set_var("RUST_LOG", "debug");
  pretty_env_logger::init();
  let listener = create_listener();
  let message_handler = MessageHandler::new();
  let mut server = Server::new(listener, message_handler);
  server.listen();
}

