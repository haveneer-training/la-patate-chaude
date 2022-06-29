use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RecoverSecretInput {
    pub word_count: usize,
    pub letters: String,
    pub tuple_sizes: Vec<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecoverSecretOutput {
    pub secret_sentence: String,
}