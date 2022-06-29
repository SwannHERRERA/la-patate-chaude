use crate::exchanger::Exchanger;
use crate::game::Game;
use std::io::Write;
use crate::message_handler::MessageHandler;
use std::net::{SocketAddr, TcpListener};
use std::sync::mpsc;
use std::thread::{self, JoinHandle};
use log::{info, trace, debug};
use shared::config::{PORT, IP};
use shared::message::Message;

pub struct Server {
  listener: TcpListener,
  game: Game,
}

impl Server {
  pub fn new(listener: TcpListener, game: Game) -> Server {
    Server { listener, game }
  }

  pub fn listen(&mut self) {
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    let (tx, rx) = mpsc::channel::<Message>();
    handles.push(self.listen_broadcast(rx));

    for stream in self.listener.incoming() {
      let stream = stream.unwrap();
      debug!("{:?}", stream);
      let stream_copy = stream.try_clone().unwrap();
      info!("players {:?}", self.game.players.get_players());
      let message_handler = MessageHandler::new_from_game(&self.game);
      let tx = tx.clone();
      let game = self.game.clone();
      let handle = thread::spawn(move || {
        let mut exchanger = Exchanger::new(message_handler, tx, game);
        exchanger.hold_communcation(stream_copy);
      });
      handles.push(handle);
    }
    for handle in handles {
      handle.join().unwrap();
    }
  }

  fn listen_broadcast(&self, rx: mpsc::Receiver<Message>) -> JoinHandle<()> {
    let players = self.game.players.players.clone();
    info!("players {:?}", self.game.players.get_players());
    let broadcast_reciever = thread::spawn(move || loop {
      match rx.recv() {
        Ok(msg) => {
          let mut players = players.lock().unwrap();
          info!("rx recieve : {:?}", msg);
          for player in players.iter_mut() {
            let response = serde_json::to_string(&msg).unwrap();
            let response = response.as_bytes();
            let response_size = response.len() as u32;
            let response_length_as_bytes = response_size.to_be_bytes();
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