use std::{sync::{Mutex, Arc}, time::Instant};

use shared::{challenge::{ChallengeType, GameType}, config};

use crate::player::PlayerList;



#[derive(Debug, Clone)]
pub struct Game {
    pub players: PlayerList,
    pub game_type: GameType,
    pub next_target: String,
    pub current_challenge: Arc<Mutex<Option<ChallengeType>>>,
    pub current_round: u32,
    pub round_timer: Arc<Mutex<Option<Instant>>>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            players: PlayerList::new(),
            next_target: String::new(),
            game_type: config::GAME_TYPE,
            current_challenge: Arc::new(Mutex::new(None)),
            current_round: 0,
            round_timer: Arc::new(Mutex::new(None)),
        }
    }
}
