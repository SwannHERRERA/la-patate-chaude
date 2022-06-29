use hashcash::{dto::{MD5HashCash, MD5HashCashInput, MD5HashCashOutput}, hashcash::Hashcash};
use serde::{Deserialize, Serialize};

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
        Hashcash::solve(self.0.message.clone(), self.0.complexity)
    }

    fn verify(&self, _: Self::Output) -> bool {
        todo!()
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