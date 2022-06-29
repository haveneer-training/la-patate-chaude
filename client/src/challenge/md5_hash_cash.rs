use common::challenge::models_md5_hash_cash::{MD5HashCashInput, MD5HashCashOutput};

pub fn md5_challenge_resolver(input: MD5HashCashInput) -> MD5HashCashOutput{
    /*let mut seed = 0;
    let mut hashcode = String::new();
    let mut message = challenge.message.clone();
    let mut hash = md5::compute(challenge.message.as_bytes());
    let mut hash_string = String::from_utf8(hash.as_ref().to_vec()).unwrap();
    let mut hash_string_bytes = hash_string.as_bytes();
    let mut hash_string_bytes_vec = hash_string_bytes.to_vec();
    let mut hash_string_bytes_vec_len = hash_string_bytes_vec.len();
    let mut hash_string_bytes_vec_len_div_2 = hash_string_bytes_vec_len / 2;
    let mut hash_string_bytes_vec_len_mod_2 = hash_string_bytes_vec_len % 2;
    let mut hash_string_bytes_vec_len_div_2_plus_1 = hash_string_bytes_vec_len_div_2 + 1;
    let mut hash_string_bytes_vec_len_div_2_plus_1_minus_1 = hash_string_bytes_vec_len_div_2_plus_1 - 1;
    let mut hash_string_bytes_vec_len_div_2_plus_1_minus_1_minus_1 = hash_string_bytes_vec_len_div_2_plus_1_minus_1 - 1;
    let mut hash_string_bytes_vec_len_div_2_plus_1_minus_1_minus_1_minus_1 = hash_string_bytes_vec_len_div_2_plus_1_minus_1_minus_1 - 1;
    let mut hash_string_bytes_vec_len_div_2_plus_1_minus_1_minus_1_minus_1_minus_1 = hash_string_bytes_vec_len_div_2_plus_1_minus_1_minus_1_minus_1 - 1;*/
    return MD5HashCashOutput{seed: 0, hashcode: String::from("")};
}