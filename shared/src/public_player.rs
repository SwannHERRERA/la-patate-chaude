use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicPlayer {
  pub name: String,
  pub stream_id: String,
  pub score: i32,
  pub steps: u32,
  pub is_active: bool,
  pub total_used_time: f64,
}

impl PublicPlayer {
  pub fn new(name: String) -> PublicPlayer {
    PublicPlayer {
      name,
      stream_id: Uuid::new_v4().to_string(),
      score: 0,
      steps: 0,
      is_active: true,
      total_used_time: 0.0,
    }
  }
  pub fn make_inactive(&mut self) {
    self.is_active = false;
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_new_public_player() {
    let player = PublicPlayer::new("Test".to_string());
    assert_eq!(player.name, "Test");
    assert_eq!(player.score, 0);
    assert_eq!(player.steps, 0);
    assert_eq!(player.is_active, true);
    assert_eq!(player.total_used_time, 0.0);
  }
  #[test]
  fn test_set_active() {
    let mut player = PublicPlayer::new("Test".to_string());
    player.make_inactive();
    assert_eq!(player.is_active, false);
  }
}