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
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    let (tx, rx) = mpsc::channel::<Message>();

    handles.push(self.listen_broadcast(rx));
    for message in self.listener.incoming() {
      debug!("message={message:?}");
      let message_handler = self.message_handler.clone();
      let tx = tx.clone();
      let handle = thread::spawn(move || {
        let mut exchanger = Exchanger::new(message_handler, tx);
        exchanger.hold_communcation(message);
      });
      handles.push(handle);
    }
    for handle in handles {
      handle.join().unwrap();
    }
  }

  fn listen_broadcast(&mut self, rx: mpsc::Receiver<Message>) -> JoinHandle<()> {
    let broadcast_reciever = thread::spawn(move || loop {
      match rx.recv() {
        Ok(msg) => {
          info!("rx recieve : {:?}", msg);
          // TODO send to all clients
        }
        Err(err) => {
          info!("rx recieve error : {:?}", err);
          break;
        }
      }
    });
    broadcast_reciever
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