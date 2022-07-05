use std::collections::{HashSet};

use serde::{Deserialize, Serialize};

use hashcash::{
    dto::{MD5HashCash, MD5HashCashInput, MD5HashCashOutput},
    hashcash::Hashcash,
};
use recover_secret::challenge_generator::validate_challenge;
use recover_secret::challenge_resolve::{
    solve_secret_sentence_challenge, solve_secret_sentence_challenge_cheat,
    solve_secret_string_challenge, solve_secret_string_challenge_cheat,
};
use recover_secret::models::{RecoverSecret, RecoverSecretInput, RecoverSecretOutput};

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

pub trait DictionaryChallenge: Challenge {
    fn solve_secret(&self, dictionary_hashmap: &HashSet<String>) -> Self::Output;
    fn solve_cheat(&self) -> Self::Output;
    fn solve_secret_cheat(&self) -> Self::Output;
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
        Hashcash::solve(self.0.message.clone(), self.0.complexity)
    }

    fn verify(&self, result: Self::Output) -> bool {
        Hashcash::verify(result.hashcode.clone(), self.0.complexity)
    }
}

impl DictionaryChallenge for RecoverSecret {
    fn solve_secret(&self, dictionary_hashmap: &HashSet<String>) -> Self::Output {
        solve_secret_sentence_challenge(&self.0, dictionary_hashmap)
    }

    fn solve_cheat(&self) -> Self::Output {
        solve_secret_string_challenge_cheat(&self.0)
    }

    fn solve_secret_cheat(&self) -> Self::Output {
        solve_secret_sentence_challenge_cheat()
    }
}

impl Challenge for RecoverSecret {
    type Input = RecoverSecretInput;
    type Output = RecoverSecretOutput;

    fn name() -> String {
        "RecoverSecret".to_string()
    }

    fn new(input: Self::Input) -> Self {
        RecoverSecret(input)
    }

    fn solve(&self) -> Self::Output {
        solve_secret_string_challenge(&self.0)
    }

    fn verify(&self, result: Self::Output) -> bool {
        validate_challenge(&self.0, &result)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ChallengeAnswer {
    MD5HashCash(MD5HashCashOutput),
    RecoverSecret(RecoverSecretOutput),
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
pub enum ChallengeType {
    MD5HashCash(MD5HashCash),
    RecoverSecret(RecoverSecret),
}

#[derive(Debug, Clone)]
pub enum GameType {
    HashCash,
    RecoverSecret,
}