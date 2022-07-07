use std::time::Duration;

use crate::challenge::GameType;

pub const IP: [u8; 4] = [127, 0, 0, 1];
pub const PORT: u16 = 7878;
pub const LOG_LEVEL: &'static str = "debug";
pub const CHALLENGE_TYPE: &'static str = "hashcash";
pub const ROUND_DURATION: Duration = Duration::from_secs(3);
pub const GAME_TYPE: GameType = GameType::RecoverSecret;