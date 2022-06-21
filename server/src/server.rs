use crate::exchanger::Exchanger;
use crate::message_handler::MessageHandler;
use std::io::Write;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex, mpsc};
use std::thread::{self, JoinHandle};
use log::{info, debug, trace};
use shared::config::{PORT, IP};
use shared::message::Message;

pub struct Server {
  listener: TcpListener,
  message_handler: Arc<Mutex<MessageHandler>>,
  streams: Vec<Arc<TcpStream>>,
}

impl Server {
  pub fn new(listener: TcpListener, message_handler: MessageHandler, streams: Vec<Arc<TcpStream>>) -> Server {
    Server { listener, message_handler: Arc::new(Mutex::new(message_handler)), streams }
  }

  pub fn listen(&mut self) {
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    let (tx, rx) = mpsc::channel::<Message>();

    handles.push(self.listen_broadcast(rx));

    for message in self.listener.incoming() {
      let message = message.unwrap();
      self.streams.push(Arc::new(message.try_clone().unwrap()));
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

  fn listen_broadcast(&self, rx: mpsc::Receiver<Message>) -> JoinHandle<()> {
    let message_handler = self.message_handler.clone();
    let s = vec![];
    let broadcast_reciever = thread::spawn(|| loop {
      match rx.recv() {
        Ok(msg) => {
          info!("rx recieve : {:?}", msg);
          let message_handler = message_handler.lock().unwrap();
          for stream in self.streams {
            let response = serde_json::to_string(&msg).unwrap();
            let response = response.as_bytes();
            let response_size = response.len() as u32;
            let response_length_as_bytes = response_size.to_be_bytes();
            let result = stream.write(&[&response_length_as_bytes, response].concat());
            trace!("byte write : {:?}, ", result);
          }
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