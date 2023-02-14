extern crate data_encoding;
extern crate rand;
extern crate md5;
use data_encoding::BASE64;
use rand::{thread_rng, Rng, RngCore};
use std::iter;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use rand::prelude::*;
use md5::compute;

fn generate_seed_md5(num_bits: usize) -> u64 {
    let entropy_size = (num_bits + 7) / 8;
    let mut entropy = vec![0u8; entropy_size];
    thread_rng().fill_bytes(&mut entropy);

    let seed = BASE64.encode(&entropy);
    let mut hasher = md5::Context::new();
    hasher.consume(seed.as_bytes());
    let md5 = hasher.compute();
    let seed = format!("{:x}", md5);

    let seed = &seed[..num_bits / 4];
    let seed = u64::from_str_radix(seed, 16).unwrap();

    seed
}

fn check_seed(seed: String, message: String, hash: String) -> bool{
    let message_to_test = format!("{}{}", seed.to_uppercase(), message);
    let digest = md5::compute(message_to_test);
    let hash_to_test = format!("{:x}", digest);
    hash_to_test.to_uppercase() == hash
}

fn solve(message: String, complexity: u32, hash: String) -> u64 {
    let mut seed:u64 = 0;
    let complexity_usize: usize = complexity as usize;
    while !check_seed(format!("{:016x}",seed).to_string(), message.clone(), hash.clone()){
        seed = generate_seed_md5(complexity_usize);
        println!("{:016x}", seed)
    }
    seed
}

fn main() {

    // Test Generate seed
    // let seed = generate_seed_md5(9);
    // println!("Seed: {:016x}", seed);

    // Test check-has
    // let hash = "00441745D9BDF8E5D3C7872AC9DBB2C3".to_string();
    // let message = "hello".to_string();
    // let seed = "000000000000034C".to_string();
    // println!("My HashCode Generated by MD5 is: {}", check_seed(seed.clone(),message.clone(), hash.clone()));

    // Test Solve
    let hash = "00441745D9BDF8E5D3C7872AC9DBB2C3".to_string();
    let message = "hello".to_string();
    let complexity = 9;
    let seed = solve(message.clone(), complexity.clone(), hash.clone());

    print!("Found: {:016x}", seed);
}
