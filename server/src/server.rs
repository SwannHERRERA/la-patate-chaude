use crate::exchanger::Exchanger;
use crate::player::Player;
use std::io::Write;
use crate::game::Game;
use crate::message_handler::MessageHandler;
use std::net::{SocketAddr, TcpListener};
use std::sync::mpsc;
use std::thread::{self, JoinHandle};
use log::{info, trace, debug};
use shared::config::{PORT, IP};
use shared::message::{MessageType, ResponseType};
use shared::public_player::PublicPlayer;

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
    let (tx, rx) = mpsc::channel::<MessageType>();
    handles.push(self.listen_message(rx));

    for stream in self.listener.incoming() {
      let stream = stream.unwrap();
      debug!("{:?}", stream);
      let stream_id = stream.peer_addr().unwrap().to_string();
      let stream_clone = stream.try_clone().unwrap();
      let public_player = PublicPlayer::new(stream_id.clone(), stream_id);
      let player = Player::new(public_player, stream_clone);
      self.game.players.add_player(player);
      info!("players {:?}", self.game.players.get_players());
      let message_handler = MessageHandler::new_from_game(&self.game);
      let tx = tx.clone();
      let game = self.game.clone();
      let handle = thread::spawn(move || {
        let mut exchanger = Exchanger::new(message_handler, tx, game);
        exchanger.hold_communcation(stream);
      });
      handles.push(handle);
    }
    for handle in handles {
      handle.join().unwrap();
    }
  }

  fn listen_message(&self, rx: mpsc::Receiver<MessageType>) -> JoinHandle<()> {
    let players = self.game.players.players.clone();
    info!("players {:?}", self.game.players.get_players());
    let broadcast_reciever = thread::spawn(move || loop {
      match rx.recv() {
        Ok(msg) => {
          debug!("rx: {:?}", msg);
          match msg.message_type {
            ResponseType::Broadcast => {
              let mut players = players.lock().unwrap();
              trace!("players {:?}", players);
              for player in players.iter_mut() {
                debug!("send to {:?}", player.info_public.stream_id);
                let response = serde_json::to_string(&msg.message).unwrap();
                let response = response.as_bytes();
                let response_size = response.len() as u32;
                let response_length_as_bytes = response_size.to_be_bytes();
                let result = player.tcp_stream.write(&[&response_length_as_bytes, response].concat());
                
                trace!("byte write : {:?}, ", result);
              }
            },
            ResponseType::Unicast { client_id } => {
              let mut players = players.lock().unwrap();
              trace!("players: {:?}", players);
              let player = players.iter_mut().find(|p| p.info_public.stream_id == client_id).unwrap();
              let response = serde_json::to_string(&msg.message).unwrap();
              let response = response.as_bytes();
              let response_size = response.len() as u32;
              let response_length_as_bytes = response_size.to_be_bytes();
              let result = player.tcp_stream.write(&[&response_length_as_bytes, response].concat());
              trace!("byte write : {:?}, ", result);
            },
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