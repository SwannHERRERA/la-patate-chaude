use std::sync::{Mutex, Arc};

use shared::{challenge::{ChallengeType}, public_player::PublicPlayer};

use crate::player::{PlayerList, Player};

#[derive(Debug, Clone)]
pub struct Game {
  pub players: PlayerList,
  pub challenge: Arc<Mutex<Option<ChallengeType>>>,
  challenge_type: String,
}

impl Game {
  pub fn new(challenge_type: String) -> Game {
    let players = PlayerList::new();
    let challenge = Arc::new(Mutex::new(None));
    Game {
      players,
      challenge,
      challenge_type,
    }
  }
  pub fn add_player(&mut self, player: Player) {
    self.players.add_player(player);
  }

  pub fn get_players(&self) -> Vec<PublicPlayer> {
    self.players.get_players()
  }

  pub fn get_challenge(&self) -> Option<ChallengeType> {
    self.challenge.lock().unwrap().clone()
  }
  pub fn set_challenge(&self, challenge: ChallengeType) {
    self.challenge.lock().unwrap().replace(challenge);
  }
}
// match challenge_type.as_str() {
//    hashcash => {
//     let challenge = Arc::new(Mutex::new(Some(ChallengeType::MD5HashCash(()))));
//     Game { players, challenge }
//   }
//   _ => panic!("Challenge Not implemented")
// }