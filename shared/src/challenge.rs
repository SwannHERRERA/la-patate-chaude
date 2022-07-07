use std::collections::{HashSet};

use serde::{Deserialize, Serialize};

use hashcash::{
    dto::{MD5HashCash, MD5HashCashInput, MD5HashCashOutput},
    hashcash::Hashcash,
};
use monstrous_maze::challenge_generator::validate_maze_challenge;
use monstrous_maze::challenge_resolve::MonstrousMazeResolver;
use monstrous_maze::models::{MonstrousMaze, MonstrousMazeInput, MonstrousMazeOutput};
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

impl Challenge for MonstrousMaze{
    type Input = MonstrousMazeInput;
    type Output = MonstrousMazeOutput;

    fn name() -> String {
        "MonstrousMaze".to_string()
    }

    fn new(input: Self::Input) -> Self {
        MonstrousMaze(input)
    }

    fn solve(&self) -> Self::Output {
        MonstrousMazeResolver::resolve_monstrous_maze_challenge(&self.0)
    }

    fn verify(&self, answer: Self::Output) -> bool {
        validate_maze_challenge(&self.0, &answer)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ChallengeAnswer {
    MD5HashCash(MD5HashCashOutput),
    RecoverSecret(RecoverSecretOutput),
    MonstrousMaze(MonstrousMazeOutput),
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
    MonstrousMaze(MonstrousMaze),
}

#[derive(Debug, Clone)]
pub enum GameType {
    HashCash,
    RecoverSecret,
    MonstrousMaze,
}

impl From<&str> for GameType {
    fn from(s: &str) -> Self {
        match s {
            "hash-cash" => GameType::HashCash,
            "recover-secret" => GameType::RecoverSecret,
            "monstrous-maze" => GameType::MonstrousMaze,
            _ => panic!("Unknown game type"),
        }
    }
}


pub fn get_name_of_challenge_type(game_type: &GameType) -> String {
    match game_type {
        GameType::HashCash => MD5HashCash::name(),
        GameType::RecoverSecret => RecoverSecret::name(),
        GameType::MonstrousMaze => MonstrousMaze::name(),
    }
}
