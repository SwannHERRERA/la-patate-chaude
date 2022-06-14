use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum SubscribeError {
    AlreadyRegistered,
    InvalidName,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChallengeOutput;

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicPlayer {
    pub name: String,
    pub stream_id: String,
    pub score: i32,
    pub steps: u32,
    pub is_active: bool,
    pub total_used_time: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChallengeAnswer {
    ChallengeName(ChallengeOutput),
}

pub struct ChallengeResult {
    pub name: ChallengeAnswer,
    pub next_target: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChallengeValue {
    Unreachable,
    Timeout,
    BadResult { used_time: f64, next_target: String },
    Ok { used_time: f64, next_target: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReportedChallengeResult {
    pub name: String,
    pub value: ChallengeValue,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MD5HashCash {
    complexity: u8,
    message: String,
}

pub type PublicLeaderBoard = Vec<PublicPlayer>;

#[derive(Serialize, Deserialize, Debug)]
pub enum SubscribeResult {
    Ok,
    Err (SubscribeError),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Hello,
    Welcome { version: u8 },
    Subscribe { name: String },
    SubscribeResult(SubscribeResult),
    PublicLeaderBoard(PublicLeaderBoard),
    Challenge {
        #[serde(rename = "MD5HashCash")]
        md5_hash_cash: MD5HashCash,
    },
    ChallengeResult {
        answer: ChallengeAnswer,
        next_target: String,
    },
    RoundSummary {
    challenge: String,
    chain: Vec<ReportedChallengeResult>,
  },
  EndOfGame {
    leader_board: Vec<PublicPlayer>,
  },
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
  fn test_message_hello_serialization() {
    let message = Message::Hello;
    let serialized = serde_json::to_string(&message).unwrap();
    assert_eq!(serialized, "\"Hello\"");
  }

  #[test]
  fn test_welcome_serialization() {
    let message = Message::Welcome { version: 1 };
    let serialized = serde_json::to_string(&message).unwrap();
    assert_eq!(serialized, "{\"Welcome\":{\"version\":1}}");
  }

  #[test]
  fn test_subscribe_serialization() {
    let message = Message::Subscribe { name: "test".to_string() };
    let serialized = serde_json::to_string(&message).unwrap();
    assert_eq!(serialized, "{\"Subscribe\":{\"name\":\"test\"}}");
  }

  #[test]
  fn test_subscribe_result_success_serialization() {
    let message = Message::SubscribeResult(SubscribeResult::Ok);
    let serialized = serde_json::to_string(&message).unwrap();
    assert_eq!(serialized, "{\"SubscribeResult\":\"Ok\"}");
  }

  #[test]
  fn test_subscribe_result_failure_serialization() {
    let message = Message::SubscribeResult(SubscribeResult::Err(SubscribeError::InvalidName));
    let serialized = serde_json::to_string(&message).unwrap();
    assert_eq!(serialized, "{\"SubscribeResult\":{\"Err\":\"InvalidName\"}}");
  }
}