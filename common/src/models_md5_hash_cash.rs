use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MD5HashCashInput {
    // complexity in bits
    pub complexity: u32,
    // message to sign
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MD5HashCashOutput {
    // Seed used to solve the challenge
    seed: u64,
    // hashcode found using seed + message
    hashcode: String,
}
