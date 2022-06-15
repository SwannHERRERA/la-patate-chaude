use std::net::{SocketAddr, TcpListener, TcpStream};
use std::io::Read;
use log::{info, debug};
use shared::config::{PORT, IP};
use shared::message::Message;


struct Server {
    listener: TcpListener,
}

impl Server {
  fn new(listener: TcpListener) -> Server {
    Server { listener }
  }

  fn listen(&self) {
    for message in self.listener.incoming() {
      debug!("message={message:?}");
      let message = self.parse_message_from_tcp_stream(message.unwrap());

      debug!("{message:?}");
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
  println!("Hello, world!");
  pretty_env_logger::init();
  let listener = create_listener();
  let server = Server::new(listener);
  server.listen();
}

