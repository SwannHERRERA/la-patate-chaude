use std::net::TcpStream;
use shared::public_player::PublicPlayer;

#[derive(Debug)]
pub struct Player {
  pub info_public: PublicPlayer,
  pub tcp_stream: TcpStream,
}

impl Player {
  pub fn new(info_public: PublicPlayer, tcp_stream: TcpStream) -> Player {
    Player {
      info_public,
      tcp_stream: tcp_stream,
    }
  }
}