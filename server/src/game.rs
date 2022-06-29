use std::sync::{Mutex, Arc};

use shared::{challenge::{ChallengeType, GameType}, config};

use crate::player::PlayerList;



#[derive(Debug, Clone)]
pub struct Game {
    pub players: PlayerList,
    pub game_type: GameType,
    pub next_target: String,
    pub current_chanllenge: Arc<Mutex<Option<ChallengeType>>>,
    pub current_round: u32,
}

impl Game {
    pub fn new() -> Game {
        Game {
            players: PlayerList::new(),
            next_target: String::new(),
            game_type: config::GAME_TYPE,
            current_chanllenge: Arc::new(Mutex::new(None)),
            current_round: 0,
        }
    }
}
