extern crate rand;

use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use crate::config;

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

impl MD5HashCashInput {
    pub fn new() -> MD5HashCashInput {
        let charset = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut rng = thread_rng();
        let complexity: u32 = rng.gen_range(5..24);
        MD5HashCashInput {
            complexity,
            message: random_string::generate(config::HASHCASH_MESSAGE_LENGTH, charset),
        }
    }
}
