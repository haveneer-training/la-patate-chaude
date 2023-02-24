use std::borrow::Borrow;

use data_encoding::BASE64;
use rand::{RngCore, thread_rng};

pub(crate) fn count_zero_bits(hash: &str, complexity: u32) -> u32 {
    let bytes = hex::decode(hash).unwrap();
    let mut num_leading_zero_bits = 0;
    for byte in &bytes {
        if byte.leading_zeros() >= complexity {
            num_leading_zero_bits += complexity;
        } else {
            num_leading_zero_bits += byte.leading_zeros();
            break;
        }
    }
    num_leading_zero_bits + 1
}

pub fn generated_md5_from_string(message: String) -> String {
    let digest = md5::compute(message);
    let hash_to_test = format!("{:x}", digest);
    hash_to_test.to_uppercase()
}