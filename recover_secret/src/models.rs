use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RecoverSecretInput {
    pub word_count: usize,
    pub letters: String,
    pub tuple_sizes: Vec<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RecoverSecretOutput {
    pub secret_sentence: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RecoverSecret(pub RecoverSecretInput);
