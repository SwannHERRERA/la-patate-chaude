use crate::exchanger::Exchanger;
use crate::game::Game;
use crate::message_handler::MessageHandler;
use crate::player::Player;
use crate::utils::send_response;
use std::net::{SocketAddr, TcpListener};
use std::sync::mpsc;
use std::thread::{self, JoinHandle};
use log::{info, debug, warn, trace};
use shared::config::{PORT, IP};
use shared::message::{MessageType, ResponseType};
use shared::public_player::PublicPlayer;

pub struct Server {
  listener: TcpListener,
  pub game: Game,
}

impl Server {
  pub fn new(listener: TcpListener, challenge_type: &str) -> Server {
    Server { listener, game: Game::new(challenge_type.to_string()) }
  }

  pub fn listen(&mut self) {
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    let (tx, rx) = mpsc::channel::<MessageType>();

    handles.push(self.listen_broadcast(rx));

    for stream in self.listener.incoming() {
      let stream = stream.unwrap();
      let stream_id = stream.peer_addr().unwrap().to_string();
      debug!("{:?}", stream);
      let stream_copy = stream.try_clone().unwrap();
      self.game.add_player(Player::new(PublicPlayer::new(stream_id.clone(), stream_id), stream));
      info!("players {:?}", self.game.get_players());
      let message_handler = MessageHandler::new(self.game.clone());
      let tx = tx.clone();
      let game_cpy = self.game.clone();
      let handle = thread::spawn(move || {
        let mut exchanger = Exchanger::new(message_handler, game_cpy, tx);
        exchanger.hold_communcation(stream_copy);
      });
      handles.push(handle);
    }
    for handle in handles {
      handle.join().unwrap();
    }
  }

  fn listen_broadcast(&self, rx: mpsc::Receiver<MessageType>) -> JoinHandle<()> {
    let mut players = self.game.players.clone();
    info!("players {:?}", self.game.get_players());
    thread::spawn(move || loop {
      match rx.recv() {
        Ok(msg) => {
          info!("Sending : {:?}", &msg);
          match msg.message_type {
            ResponseType::Broadcast => {
              let mut players = players.players.lock().unwrap();
              for player in players.iter_mut().filter(|p| p.info_public.is_active) {
                debug!("broadcast to {:?}", &player.info_public.name);
                send_response(msg.message.clone(), &player.tcp_stream);
              }
            }
            ResponseType::Unicast { client_id } => {
              trace!("unicast to {:?}", &client_id);
              let player = players.get_and_remove_player_by_stream_id(client_id);
              match player {
                Some(player) => {
                  send_response(msg.message, &player.tcp_stream);
                  players.add_player(player);
                }
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