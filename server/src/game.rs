use std::{sync::{Mutex, Arc}, time::{Duration, Instant}, collections::HashSet};

use log::{trace, error, debug};
use shared::{challenge::{ChallengeType, ReportedChallengeResult}, public_player::PublicPlayer, config};

use crate::player::{PlayerList, Player};


pub type PlayerName = String;

#[derive(Debug)]
pub struct Round {
  pub solvers: HashSet<PlayerName>,
  pub start: Instant,
  pub last_resolved: Instant,
  pub duration: Duration,
  pub acctual_player: Option<PlayerName>,
}

impl Round {
  pub fn new(duration: Duration) -> Round {
    Round {
      solvers: HashSet::new(),
      start: Instant::now(),
      last_resolved: Instant::now(),
      duration,
      acctual_player: None,
    }
  }
}

#[derive(Debug, Clone)]
pub struct Game {
  pub players: PlayerList,
  pub challenge: Arc<Mutex<Option<ChallengeType>>>,
  pub challenge_type: String,
  pub chain: Arc<Mutex<Vec<ReportedChallengeResult>>>,
  pub rounds: Arc<Mutex<Vec<Round>>>,
  pub current_round: Arc<Mutex<Option<Round>>>,
}

impl Game {
  pub fn new(challenge_type: String) -> Game {
    let players = PlayerList::new();
    let challenge = Arc::new(Mutex::new(None));
    let chain = Arc::new(Mutex::new(Vec::new()));
    let rounds = Arc::new(Mutex::new(Vec::new()));
    let current_round = Arc::new(Mutex::new(None));
    Game {
      players,
      challenge,
      challenge_type,
      chain,
      rounds,
      current_round,
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

  pub fn set_active_player(&self, name: String) {
    let mut round = self.current_round.lock().unwrap();
    debug!("set_active_player lock: {:?}", round);
    if let Some(round) = &mut *round {
      round.acctual_player = Some(name);
    }
    drop(round);
  }

  pub fn update_winner(&mut self, client_id: &str) {
    let player = self.players.get_and_remove_player_by_stream_id(client_id.to_string());
    if let Some(mut player) = player {
      let mut current_round = self.current_round.lock().unwrap();
      if let Some(current_round) = &mut *current_round {
        current_round.solvers.insert(player.info_public.name.clone());
        player.info_public.steps += 1;
        player.info_public.total_used_time += current_round.last_resolved.elapsed().as_micros() as f64;
      } else {
        error!("No current round to update winner");
      }

      self.players.add_player(player);
      trace!("players: {:?}", self.players);
    }
  }

  pub fn update_score(&self, name: &str) {
    self.players.decrease_score(name);

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

  pub fn get_player_by_name(&self, name: &str) -> Option<PublicPlayer> {
    self.players.get_player_by_name(name)
  }

  pub fn start_round(&self) {
    let current_round = Round::new(config::ROUND_DURATION);
    self.current_round.lock().unwrap().replace(current_round);
  }
}
// match challenge_type.as_str() {
//    hashcash => {
//     let challenge = Arc::new(Mutex::new(Some(ChallengeType::MD5HashCash(()))));
//     Game { players, challenge }
//   }
//   _ => panic!("Challenge Not implemented")
// }