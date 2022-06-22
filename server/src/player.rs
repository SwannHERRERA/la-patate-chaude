use std::{net::TcpStream, sync::Mutex, lazy::SyncOnceCell};
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

struct PlayerList {
  players: &'static Mutex<Vec<Player>>,
}

pub fn get_player_list() -> PlayerList {
  static INSTANCE: SyncOnceCell<Mutex<Vec<Player>>> = SyncOnceCell::new();
  PlayerList {
    players: INSTANCE.get_or_init(|| Mutex::new(Vec::new())),
  }
}

impl PlayerList {
    fn new() -> PlayerList {
        PlayerList {
            players: Vec::new(),
        }
    }

    fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    fn remove_player(&mut self, player: Player) {
        self.players.retain(|p| p.info_public.name != player.info_public.name);
    }
}