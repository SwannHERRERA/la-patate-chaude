use crate::exchanger::Exchanger;
use crate::message_handler::MessageHandler;
use crate::player::Player;
use std::io::Write;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex, mpsc};
use std::thread::{self, JoinHandle};
use log::{info, trace};
use shared::config::{PORT, IP};
use shared::message::Message;
use shared::public_player::PublicPlayer;

pub struct Server {
  listener: TcpListener,
  message_handler: Arc<Mutex<MessageHandler>>,
  players: Vec<Arc<Mutex<Player>>>,
}

impl Server {
  pub fn new(listener: TcpListener, message_handler: MessageHandler, players: Vec<Arc<Mutex<Player>>>) -> Server {
    Server { listener, message_handler: Arc::new(Mutex::new(message_handler)), players }
  }

  fn push_player(&mut self, player: Arc<Mutex<Player>>) {
    self.players.push(player);
  }

  pub fn listen(&mut self) {
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    let (tx, rx) = mpsc::channel::<Message>();

    handles.push(self.listen_broadcast(rx));

    for stream in self.listener.incoming() {
      let stream = stream.unwrap();
      let stream_copy = stream.try_clone().unwrap();
      // TODO fix issue when user is start_game
      
      self.players.push(player);
      info!("players={:?}", self.players);
      let message_handler = self.message_handler.clone();
      let tx = tx.clone();
      let handle = thread::spawn(move || {
        let mut exchanger = Exchanger::new(message_handler, tx);
        exchanger.hold_communcation(stream_copy);
      });
      handles.push(handle);
    }
    for handle in handles {
      handle.join().unwrap();
    }
  }

  fn listen_broadcast(&self, rx: mpsc::Receiver<Message>) -> JoinHandle<()> {
    let players = self.players.clone();
    let broadcast_reciever = thread::spawn(move || loop {
      info!("players {:?}", players);
      match rx.recv() {
        Ok(msg) => {
          info!("rx recieve : {:?}", msg);
          for player in &players {
            let response = serde_json::to_string(&msg).unwrap();
            let response = response.as_bytes();
            let response_size = response.len() as u32;
            let response_length_as_bytes = response_size.to_be_bytes();
            let mut player = player.lock().unwrap();
            let result = player.tcp_stream.write(&[&response_length_as_bytes, response].concat());

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