extern crate random_string;

use std::borrow::Borrow;
use std::io::prelude::*;
use std::net::TcpStream;
use recoverSecret::{generateRecoverSecretSentence, getValueIndex, validateRecoverSecretSentence};

mod recoverSecret;
mod RecoverSecretStruct;

fn main() {
}

#[cfg(test)]
mod tests {
    use ::{validateRecoverSecretSentence};
    use ::{generateRecoverSecretSentence, getValueIndex};
    use RecoverSecretStruct::RecoverSecretInput;

    #[test]
    fn should_validate_secret_sentence() {
        // First test
        let first_test_word = "C'est chou";

        let recoverSecretInput = RecoverSecretInput {
            word_count: 2,
            letters: "t cCehuCethoCeschouC'schout h".to_string(),
            tuple_sizes: Vec::from([3, 4, 5, 7, 7, 3])
        };
        assert!(validateRecoverSecretSentence(first_test_word.to_string(), recoverSecretInput.tuple_sizes.clone(), recoverSecretInput.letters.clone()));

        // Second test
        let second_test_word = "xWRvraj4fonTUmzyO25wA3lBeiM9H";
        let recoverSecretInput2 = RecoverSecretInput {
            word_count: 1,
            letters: "WvyOAlxafUzleiSOl9xayBeHTmy9xWTU5lMW4nUO5lMWRajn2BiHSRUzy5afnUz5wlexWrm5wlBWr4mAlBrUmzHxTUzwlHrfTwBeSRmzlMSRfoUOAe9S4oUiraOiramzM5w3l".to_string(),
            tuple_sizes: Vec::from([6, 8, 4, 6, 4, 7, 8, 9, 6, 9, 8, 7, 5, 7, 6, 6, 9, 5, 4, 5, 4])
        };
        assert!(validateRecoverSecretSentence(second_test_word.to_string(), recoverSecretInput2.tuple_sizes.clone(), recoverSecretInput2.letters.clone()));
    }

    #[test]
    fn should_not_validate_secret_sentence() {
        // First test
        let first_test_word = "C'set chou";
        let recoverSecretInput = RecoverSecretInput {
            word_count: 2,
            letters: "t cCehuCethoCeschouC'schout h".to_string(),
            tuple_sizes: Vec::from([3, 4, 5, 7, 7, 3])
        };
        assert!(!validateRecoverSecretSentence(first_test_word.to_string(), recoverSecretInput.tuple_sizes.clone(), recoverSecretInput.letters.clone()));

        // Second test
        let second_test_word = "xWRarvj4fonTUmzyO25wA3BleiM9H";
        let recoverSecretInput2 = RecoverSecretInput {
            word_count: 1,
            letters: "WvyOAlxafUzleiSOl9xayBeHTmy9xWTU5lMW4nUO5lMWRajn2BiHSRUzy5afnUz5wlexWrm5wlBWr4mAlBrUmzHxTUzwlHrfTwBeSRmzlMSRfoUOAe9S4oUiraOiramzM5w3l".to_string(),
            tuple_sizes: Vec::from([6, 8, 4, 6, 4, 7, 8, 9, 6, 9, 8, 7, 5, 7, 6, 6, 9, 5, 4, 5, 4])
        };
        assert!(!validateRecoverSecretSentence(second_test_word.to_string(), recoverSecretInput.tuple_sizes.clone(), recoverSecretInput.letters.clone()));
    }

    #[test]
    fn should_return_good_index_position(){
        assert_eq!(getValueIndex("Hello!".to_string(), "l".to_string()), 2);
        assert_eq!(getValueIndex("Hello!".to_string(), "H".to_string()), 0);
        assert_eq!(getValueIndex("Hello!".to_string(), "o".to_string()), 4);
    }

    #[test]
    fn should_not_return_good_index_position(){
        assert_eq!(getValueIndex("Hello!".to_string(), "j".to_string()), -1);
        assert_eq!(getValueIndex("Hello!".to_string(), "1".to_string()), -1);
    }

    #[test]
    fn should_generate_good_output1(){
        let recoverSecretInput1 = RecoverSecretInput {
            word_count: 2,
            letters: "t cCehuCethoCeschouC'schout h".to_string(),
            tuple_sizes: Vec::from([3, 4, 5, 7, 7, 3])
        };
        let mut recoverSecretOutput: String = generateRecoverSecretSentence(recoverSecretInput1.tuple_sizes.clone(), recoverSecretInput1.letters.clone()).secret_sentence.to_string();

        assert_eq!(recoverSecretOutput, "Cet 'schou");

        assert!(validateRecoverSecretSentence(recoverSecretOutput.to_string(), recoverSecretInput1.tuple_sizes.clone(), recoverSecretInput1.letters.clone()));
    }

    #[test]
    fn should_generate_good_output2(){
        let recoverSecretInput2 = RecoverSecretInput {
            word_count: 1,
            letters: "WvyOAlxafUzleiSOl9xayBeHTmy9xWTU5lMW4nUO5lMWRajn2BiHSRUzy5afnUz5wlexWrm5wlBWr4mAlBrUmzHxTUzwlHrfTwBeSRmzlMSRfoUOAe9S4oUiraOiramzM5w3l".to_string(),
            tuple_sizes: Vec::from([6, 8, 4, 6, 4, 7, 8, 9, 6, 9, 8, 7, 5, 7, 6, 6, 9, 5, 4, 5, 4])
        };

        let mut recoverSecretOutput = generateRecoverSecretSentence(recoverSecretInput2.tuple_sizes.clone(), recoverSecretInput2.letters.clone()).secret_sentence.to_string();

        assert_eq!(recoverSecretOutput, "xWvSRra4fTjnoUmzyO5w3Al2BeiM9H");

        assert!(validateRecoverSecretSentence(recoverSecretOutput.to_string(), recoverSecretInput2.tuple_sizes.clone(), recoverSecretInput2.letters.clone()));
    }
}
