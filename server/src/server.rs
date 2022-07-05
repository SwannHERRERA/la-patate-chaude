use crate::exchanger::Exchanger;
use crate::message_handler::MessageHandler;
use crate::player::PlayerList;
use crate::utils::send_response;
use std::io::Write;
use std::net::{SocketAddr, TcpListener};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self, JoinHandle};
use log::{info, debug, warn};
use shared::challenge::ChallengeType;
use shared::config::{PORT, IP};
use shared::message::{MessageType, ResponseType};

pub struct Server {
  listener: TcpListener,
  players: PlayerList,
  pub current_challenge: Arc<Mutex<Option<ChallengeType>>>,
}

impl Server {
  pub fn new(listener: TcpListener, players: PlayerList) -> Server {
    Server { listener, players, current_challenge: Arc::new(Mutex::new(None)) }
  }

  pub fn listen(&mut self) {
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    let (tx, rx) = mpsc::channel::<MessageType>();

    handles.push(self.listen_broadcast(rx));

    for stream in self.listener.incoming() {
      let stream = stream.unwrap();
      debug!("{:?}", stream);
      let stream_copy = stream.try_clone().unwrap();
      info!("players {:?}", self.players.get_players());
      let message_handler = MessageHandler::new(self.players.clone(), self.current_challenge.clone());
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

  fn listen_broadcast(&self, rx: mpsc::Receiver<MessageType>) -> JoinHandle<()> {
    let players = self.players.clone();
    info!("players {:?}", self.players.get_players());
    thread::spawn(move || loop {
      match rx.recv() {
        Ok(msg) => {
          info!("rx recieve : {:?}", &msg);
          match msg.message_type {
            ResponseType::Broadcast => {
              let mut players = players.players.lock().unwrap();
              for player in players.iter_mut() {
                send_response(msg.message.clone(), &player.tcp_stream);
              }
            }
            ResponseType::Unicast { client_id } => {
              let player = players.get_and_remove_player_by_stream_id(client_id);
              match player {
                Some(player) => send_response(msg.message, &player.tcp_stream),
                None => warn!("player not found"),
              }
            }
         };
        }
        Err(err) => {
          info!("rx recieve error : {:?}", err);
          break;
        }
      }
    })
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