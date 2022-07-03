use md5;
use md5::Digest;
use common::challenge::models_md5_hash_cash::{MD5HashCashInput, MD5HashCashOutput};

pub fn md5_challenge_resolver(input: MD5HashCashInput) -> MD5HashCashOutput {
    let mut seed: u64 = 0;
    let mut digest: Digest = md5::compute(format!("{:016X}{}", seed, input.message));
    while check_number_of_bit_at_zero(digest.as_slice(), input.complexity) == false {
        seed += 1;
        digest = md5::compute(format!("{:016X}{}", seed, input.message));
    }
    return MD5HashCashOutput { seed, hashcode: format!("{:032X}", digest) };
}

fn check_number_of_bit_at_zero(number: &[u8], expected_of_zero: u32) -> bool {

    let mut number_as_bits: u128 = number[0] as u128;
    for i in 1..number.len() {
        number_as_bits = number_as_bits << 8;
        number_as_bits += number[i] as u128;
    }
    number_as_bits = number_as_bits.reverse_bits();
    let mut number_of_zero = 0;
    while number_of_zero < expected_of_zero {
        if (number_as_bits & 0x1) == 0 {
            number_of_zero += 1;
        } else {
            return false;
        }
        number_as_bits = number_as_bits >> 1;
    }
    return true;
}