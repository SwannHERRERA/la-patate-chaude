extern crate rand;
use rand::Rng;
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
      tcp_stream
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

    pub fn get_players(&self) -> Vec<PublicPlayer> {
        self.players.lock().unwrap().iter().map(|p| p.info_public.clone()).collect()
    }

    pub fn len(&self) -> usize {
      self.players.lock().unwrap().len()
    }

    pub fn has_player_with_name(&self, name: &str) -> bool {
      self.players.lock().unwrap().iter().any(|p| p.info_public.name == name)
    }

    pub fn pick_random_player(&self) -> Option<PublicPlayer> {
      let mut rng = rand::thread_rng();
      let index = rng.gen_range(0..self.players.lock().unwrap().len());
      self.players.lock().unwrap().get(index).map(|p| p.info_public.clone())
    }

    pub fn get_and_remove_player_by_name(&self, name: &str) -> Option<Player> {
      let index = self.players.lock().unwrap().iter().position(|p| p.info_public.name == name);
      if let Some(index) = index {
        Some(self.players.lock().unwrap().remove(index))
      } else {
        None
      }
    }

    pub fn unable_player(&mut self, stream: &TcpStream) {
      let index = self.players.lock().unwrap().iter().position(|p| p.info_public.stream_id == stream.peer_addr().unwrap().to_string());
      if let Some(index) = index {
        self.players.lock().unwrap()[index].info_public.is_active = false;
      }
    }
    pub fn activate_player(&mut self, client_id: &str) {
      let index = self.players.lock().unwrap().iter().position(|p| p.info_public.stream_id == client_id);
      if let Some(index) = index {
        self.players.lock().unwrap()[index].info_public.is_active = true;
      }
    }
}
