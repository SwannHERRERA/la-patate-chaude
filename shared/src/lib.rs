use std::io::Write;
use std::process::{Command, Stdio};

use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum SubscribeError {
    AlreadyRegistered,
    InvalidName,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MD5HashCashOutput {
    pub seed: u64,
    pub hashcode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MD5HashCashInput {
    complexity: u32,
    message: String,
}

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
    MD5HashCash(MD5HashCashOutput)
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

pub type PublicLeaderBoard = Vec<PublicPlayer>;

#[derive(Serialize, Deserialize, Debug)]
pub enum SubscribeResult {
    Ok,
    Err(SubscribeError),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MD5HashCash(MD5HashCashInput);

#[derive(Serialize, Deserialize, Debug)]
pub enum ChallengeType {
    MD5HashCash(MD5HashCash)
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Hello,
    Welcome { version: u8 },
    Subscribe { name: String },
    SubscribeResult(SubscribeResult),
    PublicLeaderBoard(PublicLeaderBoard),
    Challenge(ChallengeType),
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

pub trait Challenge {
    /// Données en entrée du challenge
    type Input;
    /// Données en sortie du challenge
    type Output;
    /// Nom du challenge
    fn name() -> String;
    /// Create a challenge from the specific input
    fn new(input: Self::Input) -> Self;
    /// Résout le challenge
    fn solve(&self) -> Self::Output;
    /// Vérifie qu'une sortie est valide pour le challenge
    fn verify(&self, answer: Self::Output) -> bool;
}

impl Challenge for MD5HashCash {
    type Input = MD5HashCashInput;
    type Output = MD5HashCashOutput;

    fn name() -> String {
        "MD5HashCash".to_string()
    }

    fn new(input: Self::Input) -> Self {
        MD5HashCash(input)
    }

    fn solve(&self) -> Self::Output {
        let seed = generate_seed(self.0.complexity);
        let sum = hash_md5(format!("{:016X}", seed) + &self.0.message.to_string());
        MD5HashCashOutput { seed, hashcode: sum.to_string() }
    }

    fn verify(&self, answer: Self::Output) -> bool {
        todo!()
    }
}

fn generate_seed(complexity: u32) -> u64 {
    let mut rng = rand::thread_rng();
    let mut seed: u64 = rng.gen::<u64>();
    seed = seed << 32 - complexity;
    seed = seed >> 32 - complexity;
    seed
}

fn hash_md5(data: String) -> String {
    println!("{:?}", data);
    let mut md5_cmd = Command::new("md5sum")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    {
        let child_stdin = md5_cmd.stdin.as_mut().unwrap();
        child_stdin.write_all(data.as_bytes()).expect("Error while writing to md5 stdin");
    }

    let output = md5_cmd.wait_with_output().unwrap();
    let md5 = String::from_utf8_lossy(&output.stdout);
    md5.split(" ").next().unwrap().to_ascii_uppercase()
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