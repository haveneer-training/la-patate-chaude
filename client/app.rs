extern crate data_encoding;
extern crate hex;
extern crate md5;
extern crate rand;
extern crate rayon;

use std::convert::{TryFrom, TryInto};

use data_encoding::BASE64;
use md5::Digest;
use rand::{RngCore, thread_rng};

use hashcash::data_structures::{MD5HashCashInput, MD5HashCashOutput};
use hashcash::md5_hascash_challenge::{Challenge, Md5Challenge};
use hashcash::utils::generated_md5_from_string;

mod hashcash;


fn main() {
    let md5_hashcash_input = MD5HashCashInput {
        complexity: 9,
        message: "hello".to_string()
    };

    let md5_hashcash_challenge = Md5Challenge::new(md5_hashcash_input);
    let result = md5_hashcash_challenge.solve();

    let status = md5_hashcash_challenge.verify(&result);

    println!("Validation status: {status}")
}

#[cfg(test)]
mod tests {
    use ::{MD5HashCashInput, MD5HashCashOutput};
    use ::{Challenge, Md5Challenge};
    use hashcash::utils::{count_zero_bits, generated_md5_from_string};

    #[test]
    fn should_validate_first_seed_test() {
        let md5_hashcash_output = MD5HashCashOutput{
            seed: 368 as u64,
            hashcode: "00E749706D04FE4FC60CB007FE82C209".to_string()
        };

        let md5_hashcash_input = MD5HashCashInput {
            complexity: 9,
            message: "hello".to_string()
        };

        let md5_hashcash_challenge = Md5Challenge::new(md5_hashcash_input);

        let status = md5_hashcash_challenge.verify(&md5_hashcash_output);
        assert!(status)
    }

    #[test]
    fn should_validate_second_seed() {
        let md5_hashcash_output = MD5HashCashOutput{
            seed: 844 as u64,
            hashcode: "00441745D9BDF8E5D3C7872AC9DBB2C3".to_string()
        };

        let md5_hashcash_input = MD5HashCashInput {
            complexity: 9,
            message: "hello".to_string()
        };

        let md5_hashcash_challenge = Md5Challenge::new(md5_hashcash_input);

        let status = md5_hashcash_challenge.verify(&md5_hashcash_output);
        assert!(status)
    }

    #[test]
    fn should_not_validate_seed() {
        let md5_hashcash_output = MD5HashCashOutput{
            seed: 84 as u64,
            hashcode: "00441745D9BDF8E5D3C7872AC9DBB2C3".to_string()
        };

        let md5_hashcash_input = MD5HashCashInput {
            complexity: 9,
            message: "hello".to_string()
        };

        let md5_hashcash_challenge = Md5Challenge::new(md5_hashcash_input);

        let status = md5_hashcash_challenge.verify(&md5_hashcash_output);
        assert!(!status)
    }

    #[test]
    fn should_generated_good_seed() {
        let md5_hashcash_input = MD5HashCashInput {
            complexity: 9,
            message: "hello".to_string()
        };

        let md5_hashcash_challenge = Md5Challenge::new(md5_hashcash_input);
        let output = md5_hashcash_challenge.solve();

        assert_eq!(format!("{:016X}", output.seed), format!("{:016X}", 368 as u64));
        assert_eq!(output.hashcode, "00E749706D04FE4FC60CB007FE82C209")
    }
