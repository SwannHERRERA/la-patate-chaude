use std::{net::TcpStream, sync::{Mutex, Arc}};
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
      tcp_stream,
    }
  }
}

#[derive(Debug, Clone)]
pub struct PlayerList {
  pub players: Arc<Mutex<Vec<Player>>>,
}

impl PlayerList {
    pub fn new() -> PlayerList {
        PlayerList {
            players: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.lock().unwrap().push(player);
    }

    pub fn remove_player(&mut self, player: Player) {
        self.players.lock().unwrap().retain(|p| p.info_public.name != player.info_public.name);
    }

    pub fn get_players(&self) -> Vec<PublicPlayer> {
        self.players.lock().unwrap().iter().map(|p| p.info_public.clone()).collect()
    }

    pub fn has_player_with_name(&self, name: &str) -> bool {
      self.players.lock().unwrap().iter().any(|p| p.info_public.name == name)
    }
}
