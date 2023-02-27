use  recover_secret::utils::{pattern_word, get_value_index, is_sorted};
use ::{RecoverSecretInput, RecoverSecretOutput};

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

pub struct RecoverSecretChallenge {
    pub input: RecoverSecretInput
}

impl Challenge for RecoverSecretChallenge{
    /// Data input of the challenge
    type Input = RecoverSecretInput;
    /// Data output of the challenge
    type Output = RecoverSecretOutput;

    fn name() -> String {
        return "RecoverSecret".to_string();
    }

    fn new(input: Self::Input) -> Self {
        Self {
            input
        }
    }

    fn solve(&self) -> Self::Output {
        let mut tuple_sizes: Vec<usize> = self.input.tuple_sizes.clone() ;
        let mut letters: String = self.input.letters.clone();
        let mut result: String = "".to_string();
        let mut status = false;
        while !status{
            for i in 0..tuple_sizes.len(){

                // Get the word pattern
                let mut start_interval = 0;
                let mut end_interval = 0;
                for j in 0..i{
                    start_interval = start_interval + tuple_sizes[j];
                    end_interval = end_interval + tuple_sizes[j];
                }
                end_interval = end_interval + tuple_sizes[i];
                let mut word = &letters[start_interval..end_interval];

                let mut prec_letter:i32;
                for i in 0..word.len(){

                    let mut tmp_index = get_value_index(result.to_string(), word.chars().nth(i).unwrap().to_string());
                    // If letter of pattern do not exist, add to the end of result
                    if tmp_index==-1 {
                        result = result.to_owned() + &*word.chars().nth(i).unwrap().to_string();
                    }
                    // If letter of pattern do exist
                    else if i!=0 && tmp_index<get_value_index(result.to_string(), word.chars().nth(i-1).unwrap().to_string()){
                        // Get precedent letter index of word
                        prec_letter = get_value_index(result.to_string(), word.chars().nth(i-1).unwrap().to_string());

                        // Calculate the nex letter index of word
                        let mut prec_plus = prec_letter + 1;

                        // Get found letter index in result
                        let mut first_word_letter = get_value_index(result.to_string(), word.chars().nth(0).unwrap().to_string());

                        // Same in the two conditions but in different case
                        // - Cut from current index of result to the first letter of word
                        // - Put it just after the precedent letter index of word
                        if tmp_index < first_word_letter {
                            let mut cutted = &result[tmp_index as usize..first_word_letter as usize].to_string();
                            let new_result = result[..tmp_index as usize].to_string() + &result[first_word_letter as usize..prec_plus as usize].to_string() + &*cutted.to_string() + &result[prec_plus as usize..].to_string();
                            result = new_result;
                        }
                        else{
                            let mut cutted = &result[tmp_index as usize..prec_letter as usize].to_string();
                            let new_resutlt = result[0..tmp_index as usize].to_string() + &result[prec_letter as usize..].to_string() + &*cutted.to_string();
                            result = new_resutlt;
                        }
                    }
                    prec_letter = i as i32;
                }
            }
            status = self.verify(&RecoverSecretOutput { secret_sentence: result.clone().to_string() })
        }

        let mut output: RecoverSecretOutput = RecoverSecretOutput{
            secret_sentence: result
        };
        return output;
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        let mut test_word: String = answer.secret_sentence.clone();
        let mut tuple_sizes: Vec<usize> = self.input.tuple_sizes.clone();
        let mut letters: String = self.input.letters.clone();
        // Validate a sentence test_word with letters and tuple_sizes
        for i in 0..tuple_sizes.len(){
            let mut start_interval = 0;
            let mut end_interval = 0;
            for j in 0..i{
                start_interval = start_interval + tuple_sizes[j];
                end_interval = end_interval + tuple_sizes[j];
            }
            end_interval = end_interval + tuple_sizes[i];

            let mut word = &letters[start_interval..end_interval];
            // println!("{}", pattern_word(word, test_word));
            let status = pattern_word(word.to_string(), test_word.to_string());
            if !status {
                return false;
            }
        }
        return true;
    }
}