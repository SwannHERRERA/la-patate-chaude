use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SubscribeResult {
    Ok,
    Err (SubscribeError),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SubscribeError {
    AlreadyRegistered,
    InvalidName,
}