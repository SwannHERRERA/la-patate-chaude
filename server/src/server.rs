use crate::exchanger::Exchanger;
use crate::message_handler::MessageHandler;
use std::net::{SocketAddr, TcpListener};
use std::sync::{Arc, Mutex, mpsc};
use std::thread::{self, JoinHandle};
use log::{info, debug};
use shared::config::{PORT, IP};
use shared::message::Message;

pub struct Server {
  listener: TcpListener,
  message_handler: Arc<Mutex<MessageHandler>>,
}

impl Server {
  pub fn new(listener: TcpListener, message_handler: MessageHandler) -> Server {
    Server { listener, message_handler: Arc::new(Mutex::new(message_handler)) }
  }

  pub fn listen(&mut self) {
    let mut hanldes: Vec<JoinHandle<()>> = Vec::new();
    let (tx, rx) = mpsc::channel::<Message>();
    if let Ok(msg) = rx.try_recv() {
      info!("rx recieve : {:?}", msg);
    }
    for message in self.listener.incoming() {
      debug!("message={message:?}");
      let message_handler = self.message_handler.clone();
      let tx = tx.clone();
      let handle = thread::spawn(move || {
        let mut exchanger = Exchanger::new(message_handler, tx);
        exchanger.hold_communcation(message);
      });
      hanldes.push(handle);
    }
    for handle in hanldes {
      handle.join().unwrap();
    }
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