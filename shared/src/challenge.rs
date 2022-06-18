use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChallengeOutput;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ChallengeAnswer {
    ChallengeName(ChallengeOutput),
}

pub struct ChallengeResult {
    pub name: ChallengeAnswer,
    pub next_target: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ChallengeValue {
    Unreachable,
    Timeout,
    BadResult { used_time: f64, next_target: String },
    Ok { used_time: f64, next_target: String },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReportedChallengeResult {
    pub name: String,
    pub value: ChallengeValue,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MD5HashCash {
    complexity: u8,
    message: String,
}