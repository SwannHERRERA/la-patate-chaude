use serde::{Deserialize, Serialize};

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
  pub fn new(name: String, stream_id: String, score: i32, steps: u32, is_active: bool, total_used_time: f64) -> PublicPlayer {
    PublicPlayer {
      name,
      stream_id,
      score,
      steps,
      is_active,
      total_used_time,
    }
  }
}
