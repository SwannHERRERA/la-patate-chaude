use std::sync::{Arc, mpsc};
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::Instant;

use serde::{Deserialize, Serialize};

const NTHREADS: usize = 100;

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
    None,
    MD5HashCash(MD5HashCashOutput),
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
    MD5HashCash(MD5HashCash),
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
        let now = Instant::now();
        let seed_counter = Arc::new(AtomicU64::new(0));
        let (worker_tx, worker_rx) = mpsc::channel();
        for _ in 0..NTHREADS {
            let worker_tx = worker_tx.clone();
            let seed_counter = seed_counter.clone();
            let message = self.0.message.to_string();
            let complexity = self.0.complexity;
            thread::spawn(move || {
                loop {
                    if seed_counter.load(Ordering::Relaxed) == u64::MAX {
                        break;
                    }
                    let seed = seed_counter.fetch_add(1, Ordering::Relaxed);
                    let hash = md5::compute(format!("{:016X}", seed) + &message);
                    ;
                    let md5 = format!("{:032X}", hash);
                    if !check_hash(complexity, md5.clone()) {
                        continue;
                    }
                    worker_tx.send(MD5HashCashOutput { seed, hashcode: md5.to_string() }).unwrap();
                }
            });
        }
        let elapsed = now.elapsed();
        println!("Thread creation time elapsed 1: {:.2?}", elapsed);
        let out = worker_rx.recv().unwrap();
        let elapsed = now.elapsed();
        println!("Challenge solve time elapsed 2: {:.2?}", elapsed);
        seed_counter.store(u64::MAX, Ordering::Relaxed);
        out
    }

    fn verify(&self, answer: Self::Output) -> bool {
        todo!()
    }
}

fn check_hash(mut complexity: u32, hash: String) -> bool {
    let bit_compare = 1 << 127;
    let mut sum = u128::from_str_radix(&*hash, 16).unwrap();
    while complexity > 0 {
        if (sum & bit_compare) > 0 {
            break;
        }
        sum = sum << 1;
        complexity -= 1;
    }
    complexity == 0
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