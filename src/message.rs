use serde::{Serialize, Deserialize};

pub enum SubscribeError {
    AlreadyRegistered,
    InvalidName,
}

#[derive(Serialize, Deserialize)]
pub struct ChallengeOutput;

#[derive(Serialize, Deserialize)]
pub struct PublicPlayer {
    pub name: String,
    pub stream_id: String,
    pub score: i32,
    pub steps: u32,
    pub is_active: bool,
    pub total_used_time: f64,
}

#[derive(Serialize, Deserialize)]
pub enum ChallengeAnswer {
    ChallengeName(ChallengeOutput),
}

pub struct ChallengeResult {
    pub name: ChallengeAnswer,
    pub next_target: String,
}

#[derive(Serialize, Deserialize)]
pub enum ChallengeValue {
    Unreachable,
    Timeout,
    BadResult { used_time: f64, next_target: String },
    Ok { used_time: f64, next_target: String },
}

#[derive(Serialize, Deserialize)]
pub struct ReportedChallengeResult {
    pub name: String,
    pub value: ChallengeValue,
}

#[derive(Serialize, Deserialize)]
pub struct MD5HashCash {
  complexity: u8,
  message: String,
}

pub type PublicLeaderBoard = Vec<PublicPlayer>;
pub type SubscribeResultSuccess = ();
pub type SubscribeResultFailure = ();

#[derive(Serialize, Deserialize)]
pub enum Message {
  Hello,
  Welcome { version: u8 },
  Subscribe { name: String },
  SubscribeResult(Result<SubscribeResultSuccess, SubscribeResultFailure>),
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