use rayon::in_place_scope;
use ::{MD5HashCashInput};
use hashcash::utils::{count_zero_bits, generated_md5_from_string};
use MD5HashCashOutput;

pub trait Challenge {
    /// Data input of the challenge
    type Input;
    /// Data output of the challenge
    type Output;
    /// Name of the challenge
    fn name() -> String;
    /// Create a challenge from the specific input
    fn new(input: Self::Input) -> Self;
    /// Resolve le challenge
    fn solve(&self) -> Self::Output;
    /// Vérifie qu'une sortie est valide pour le challenge
    fn verify(&self, answer: &Self::Output) -> bool;
}

pub struct Md5Challenge {
    pub input: MD5HashCashInput
}

impl Challenge for Md5Challenge {
    /// Data input of the challenge
    type Input = MD5HashCashInput;
    /// Data output of the challenge
    type Output = MD5HashCashOutput;

    fn name() -> String {
        return "RecoverSecret".to_string();
    }

    fn new(input: Self::Input) -> Self {
        Self {
            input
        }
    }

    fn solve(&self) -> Self::Output {
        let mut message = self.input.message.clone();
        let mut complexity = self.input.complexity.clone() as usize;

        // Start with seed = 0
        let mut seed_result = 0 as u64;

        // Generate new hash code
        let mut str_seed_result_message = format!("{:016X}{}", seed_result, message);
        let mut hash_result = generated_md5_from_string(str_seed_result_message);

        // Count the number of bytes of the generated hash if has minimum complexity of input.complexity
        let mut num_zero_bits = count_zero_bits(&hash_result as &str, complexity as u32);

        // while the num_zero_bits is less than complexity, retry again the same operations
        // before with incremented seed
        while num_zero_bits < complexity as u32 {
            // Incremente seed for the checking
            seed_result += 1;
            // Calculate the hash code
            str_seed_result_message = format!("{:016X}{}", seed_result, message);
            hash_result = generated_md5_from_string(str_seed_result_message);
            // Count the number of bytes of the generated hash if has minimum complexity of input.complexity
            num_zero_bits = count_zero_bits(&hash_result as &str, complexity as u32);
        }

        // println!("Le seed_result {:016X} hash_result {} contient {} bits à zéro de complexité minimale {}.", seed_result, hash_result, num_zero_bits, complexity);

        MD5HashCashOutput{
            seed: seed_result,
            hashcode: hash_result
        }
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        let message_to_test = format!("{:016X}{}", answer.seed, self.input.message.clone());
        let hash_to_test = generated_md5_from_string(message_to_test);
        hash_to_test.to_uppercase() == answer.hashcode.clone().to_uppercase()
    }
}