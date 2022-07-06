use std::sync::{Mutex, Arc};

use log::trace;
use shared::{challenge::{ChallengeType, ReportedChallengeResult}, public_player::PublicPlayer};

use crate::player::{PlayerList, Player};

#[derive(Debug, Clone)]
pub struct Game {
  pub players: PlayerList,
  pub challenge: Arc<Mutex<Option<ChallengeType>>>,
  pub challenge_type: String,
  pub chain: Arc<Mutex<Vec<ReportedChallengeResult>>>,
}

impl Game {
  pub fn new(challenge_type: String) -> Game {
    let players = PlayerList::new();
    let challenge = Arc::new(Mutex::new(None));
    let chain = Arc::new(Mutex::new(Vec::new()));
    Game {
      players,
      challenge,
      challenge_type,
      chain
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

  pub fn add_point(&mut self, client_id: &str) {
    let player = self.players.get_and_remove_player_by_stream_id(client_id.to_string());
    if let Some(mut player) = player {
      player.info_public.steps += 1;
      // todo use a bool per user for know if they have played
      self.players.add_player(player);
      trace!("players: {:?}", self.players);
    }
  }

  pub fn get_chain(&self) -> Vec<ReportedChallengeResult> {
    self.chain.lock().unwrap().clone()
  }

  pub fn push_reported_challenge_result(&mut self, result: ReportedChallengeResult) {
    self.chain.lock().unwrap().push(result);
  }

  pub fn get_last_chain_result(&self) -> Option<ReportedChallengeResult> {
    self.chain.lock().unwrap().last().cloned()
  }
}
// match challenge_type.as_str() {
//    hashcash => {
//     let challenge = Arc::new(Mutex::new(Some(ChallengeType::MD5HashCash(()))));
//     Game { players, challenge }
//   }
//   _ => panic!("Challenge Not implemented")
// }