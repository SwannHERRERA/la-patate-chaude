use std::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub struct PublicPlayer {
  pub name: String,
  pub stream_id: String,
  pub score: i32,
  pub steps: u32,
  pub is_active: bool,
  pub total_used_time: f64,
}

impl PublicPlayer {
  pub fn new(name: String, stream_id: String) -> PublicPlayer {
    PublicPlayer {
      name,
      stream_id,
      score: 0,
      steps: 0,
      is_active: false,
      total_used_time: 0.0,
    }
  }
  pub fn make_active(&mut self, name: &str) {
    self.is_active = true;
    self.name = name.to_string();
  }
}

impl Debug for PublicPlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      if self.is_active == false {
        write!(f, "Player Inactive: {}\n", self.stream_id)?
      }
      write!(f, "{}: {}\tstep: {}, total time: {} | {}\n", self.name, self.score, self.steps, self.total_used_time, self.stream_id)
    }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_new_public_player() {
    let player = PublicPlayer::new("Test".to_string(), "127.0.0.1:1234".to_string());
    assert_eq!(player.name, "Test");
    assert_eq!(player.score, 0);
    assert_eq!(player.steps, 0);
    assert_eq!(player.is_active, false);
    assert_eq!(player.total_used_time, 0.0);
  }
  #[test]
  fn test_set_active() {
    let mut player = PublicPlayer::new("Test".to_string(), "127.0.0.1:1234".to_string());
    player.make_active("Test");
    assert_eq!(player.is_active, true);
  }
}
