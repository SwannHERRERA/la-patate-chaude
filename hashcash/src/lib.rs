use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MD5HashCashOutput {
    pub seed: u64,
    pub hashcode: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MD5HashCashInput {
    pub complexity: u32,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MD5HashCash(pub MD5HashCashInput);
