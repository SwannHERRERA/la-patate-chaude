use std::sync::{Arc, Mutex};
use std::net::TcpStream;
use shared::public_player::PublicPlayer;

pub struct Player {
  info_public: PublicPlayer,
  tcp_stream: Arc<Mutex<TcpStream>>,
}

impl Player {
  pub fn new(info_public: PublicPlayer, tcp_stream: TcpStream) -> Player {
    Player {
      info_public,
      tcp_stream: Arc::new(Mutex::new(tcp_stream)),
    }
  }
}